#![allow(unused_variables)]

// 立方体卫星
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

type Message = String;

// 地面站
struct GroundStation {}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: MailBox { messages: vec![] },
        }
    }

    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

// 返回立方体卫星id的动态数组
fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id); // 短存活期变量的所有权转移给其他函数
        println!("t0: {:?}", sat);

        base.send(&mut sat, Message::from("Hello"));

        println!("t1: {:?}", sat);

        let msg = sat.recv();

        println!("t2: {:?}", sat);

        println!("msg: {:?}", msg);
        println!("==================");
    }
}
