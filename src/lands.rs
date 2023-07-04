#[derive(Debug)]
pub struct Grassland{
    size:u32,
}
impl Grassland{
    fn explore(size:u32)-> Self{//参数不带self的是new，常用来作构造函数
        Self{
            size:size,
        }
    }
    fn reclamation(&self)-> Farmland{//参数为self的称作关联函数的方法
        Farmland{
            size:self.size
        }
    }
}

pub struct Farmland{
    size:u32,
}
pub struct Forest{

}
pub struct Plateau{

}
pub struct Wasteland{

}