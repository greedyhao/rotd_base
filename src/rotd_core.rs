enum ROTDState {
    Idle,
    AuthPW,
    AuthOk,
    AuthFail,
    SetPW,
    ConfirmPW,
}

pub struct ROTDCore {
    state: ROTDState,
    ctrl: ROTDCtrl,
    tick: u32,
    tick_bak: u32,
    password: PassWord,
    knock_buf: PassWord,
    knock_cnt: u8,
    flag: u8,
}

pub type ReadPwCtrl = fn(&mut PassWord);
pub type WritePwCtrl = fn(&PassWord);
pub type OpenDoorCtrl = fn();
pub type BuzzerCtrl = fn(bool);

pub struct ROTDCtrl {
    read_pw: ReadPwCtrl,
    write_pw: WritePwCtrl,
    open_door: OpenDoorCtrl,
    buzzer: BuzzerCtrl,
}

impl ROTDCtrl {
    pub fn new(
        read_pw: ReadPwCtrl,
        write_pw: WritePwCtrl,
        open_door: OpenDoorCtrl,
        buzzer: BuzzerCtrl,
    ) -> Self {
        ROTDCtrl {
            read_pw,
            write_pw,
            open_door,
            buzzer,
        }
    }
}

pub type PassWord = [u8; 6];

impl ROTDCore {
    pub fn new(ctrl: ROTDCtrl) -> Self {
        ROTDCore {
            state: ROTDState::Idle,
            ctrl,
            tick: 0,
            tick_bak: 0,
            password: [0; 6],
            knock_buf: [0; 6],
            knock_cnt: 0,
            flag: 0,
        }
    }
    pub fn open_signal(&mut self) {
        self.state = ROTDState::AuthPW;
        self.knock_cnt = 0;
        self.run();
    }
    pub fn knock_signal(&mut self, part: u8) {
        self.knock_buf[self.knock_cnt as usize] = part;
        self.knock_cnt += 1;
        self.run();
    }
    pub fn setpw_signal(&mut self) {
        match self.state {
            ROTDState::AuthPW => {
                self.state = ROTDState::SetPW;
                self.knock_cnt = 0;
            }
            _ => {}
        }
        self.run();
    }
    pub fn tick_500ms_signal(&mut self) {
        self.tick += 1;
        self.run();
    }
    fn run(&mut self) {
        match self.state {
            ROTDState::Idle => {
                self.tick = 0;
                self.password = [1, 1, 1, 1, 1, 1]; // default
                self.knock_buf.fill(0);
                self.knock_cnt = 0;
                self.flag = 0;
            }
            ROTDState::AuthPW => {
                if self.knock_cnt == self.password.len() as u8 {
                    if self.knock_buf == self.password {
                        self.state = ROTDState::AuthOk;
                    } else {
                        self.state = ROTDState::AuthFail;
                    }
                }
            }
            ROTDState::AuthOk => {
                (self.ctrl.open_door)();
                self.state = ROTDState::Idle;
            }
            ROTDState::AuthFail => {
                if self.flag == 0 {
                    (self.ctrl.buzzer)(true);
                    self.flag = 1;
                    self.tick_bak = self.tick;
                } else if self.tick_bak + 2 <= self.tick {
                    (self.ctrl.buzzer)(false);
                    self.flag = 0;
                    self.state = ROTDState::Idle;
                }
            }
            ROTDState::SetPW => {
                if self.knock_cnt == self.password.len() as u8 {
                    self.password = self.knock_buf;
                    self.state = ROTDState::ConfirmPW;
                }
            }
            ROTDState::ConfirmPW => {}
        }

        // timeout
        if self.tick > 2 * 60 * 2 {
            self.state = ROTDState::Idle;
        }
    }
}
