use std::f64::consts::PI;

fn main() {
    //1 TrafficLight
    let red = TrafficLight::Red;
    let num:u8 = red.get_time();
    println!("Time of {:?} light is {}s", red, num);
    //2 check overflow
    let num_list = [10,20,30];
    let num = sum_u32(&num_list);
    println!("{:?}",num.unwrap());
    //3 do area calculation
    let geometry =  Geometry {
        shape: Shape::ETriangle,
        side: 10.0,
        radius: 0.0
    };
    print_cal(&geometry)
}

/*
 *为枚举交通信号实现一个trait，trait里包含一个返回时间的方法，
 *不同灯持续时间不同
 */
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green
}
trait GetTime {
    fn get_time(&self) -> u8;
}
//implement GetTime for enum TrafficLight with match, return time by specific TrafficLight type
impl GetTime for TrafficLight {
    fn get_time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 30
        }
    }
}
/*
 *实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，
 *返回类型为Option<u32>，溢出时返回None
 */
//use method check_add in Option check if return overflows when add new num
fn sum_u32(list_u32: &[u32]) -> Option<u32> {
    let mut result:Option<u32> = Some(0);
    for num in list_u32 {
        result = result.unwrap().checked_add(*num);
        if result.is_none() {
            println!("overflow!");
            break;
        }
    }
    return result;
}
/*
 *实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，
 *比如圆形，三角形，正方形，需要用到泛型和泛型约束
 */
enum Shape {
    Circle,
    ETriangle,
    Square
}
struct Geometry {
    shape:Shape,
    side:f64,
    radius:f64
}
pub trait Calculate {
    fn calculate(&self) -> f64;
}
//implement Calculate for struct Geometry, match a shape for the specific method
impl Calculate for Geometry {
    fn calculate(&self) -> f64 {
        let square_root_3:f64 = 1.73205;
        match &self.shape {
            Shape::Circle => (self.radius * self.radius) * PI,
            Shape::ETriangle => self.side * (self.side * square_root_3 / 2.0) / 2.0,
            Shape::Square => (self.side * self.side)
        }
    }
}
//use generics to limit input data type
pub fn print_cal<T: Calculate>(item: &T) {
    println!("{}", item.calculate())
}