fn main() {
    //定义sum未溢出的数组array1
    let array1: [u32; 6] = [2,55,33,24,64,12];

    //定义sum溢出的数组array2
    let array2 = [u32::MAX,2,3];

    //计算结果
    let res1 = ArraySum(&array1);
    let res2 = ArraySum(&array2);

    //判断计算结果
     match_res(res1);
     match_res(res2);
}

//定义函数返回Option类型计算集合sum结果
fn ArraySum(array:&[u32]) -> Option<u32> {
    let mut x = array.iter();

    //运用try_fold方法，若结果未溢出则返回计算后的结果，若溢出则返回None
    let res = x.try_fold(0_u32,|acc, &y| acc.checked_add(y));
    return res;
}

//定义函数判断sum是否溢出
fn match_res(res:Option<u32>) {
    match res {
        Some(array) => println!("sum = {}",array),
        None => println!("None"),
    }
}



