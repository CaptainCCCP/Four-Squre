#[derive(Debug)]
pub enum LandType {
    Grassland,
    Farmland,
}
pub struct Grassland{
    // 大小，building承载量
    size:u32,
    //肥沃度，小麦产出量
    fertility:u32,

}
impl Grassland{
    //新建一个Grassland
    pub fn new(size:u32,fertility:u32) -> Self{//参数不带self的是new，常用来作构造函数
        //新建一片土地后开始产出
        Self{
            size:size,
            fertility:fertility,
        }
    }
    //开垦,将其类型转换为Farmland
    fn reclamation(&mut self)-> Farmland{//参数为self的称作关联函数的方法
        Farmland{
            size:self.size
        }
    }
    //更新属性
    fn update_fertility(&mut self,value:u32) {
        self.fertility = value;
    }
    //产出
    fn produce(&self) -> u32{
        self.fertility
    }
    //展示size
    pub fn show_size(&self) -> u32{
        self.size
    }
}

pub struct Farmland{
    size:u32,
}
// pub struct Forest{

// }
// pub struct Plateau{

// }
// pub struct Wasteland{

// }