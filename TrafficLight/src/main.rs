fn main() {
    //设定红绿黄灯
    let Redlight = TrafficLight::Red;
    let Greenlight = TrafficLight::Green;
    let Yellowlight = TrafficLight::Yellow;

    //输入结果
    println!("The {:?}light time is {:?}s.", Redlight, Redlight.time());
    println!("The {:?}light time is {:?}s.", Greenlight, Greenlight.time());
    println!("The {:?}light time is {:?}s.", Yellowlight, Yellowlight.time());
}

//定义trait
trait Time {
    fn time(&self) -> u8 ;
}

//定义TrafficLight类
#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

//定义返回时交通信号灯持续时间的函数
impl Time for TrafficLight{
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,

            TrafficLight::Green => 50,

            TrafficLight::Yellow => 3,
        }
    }
}

