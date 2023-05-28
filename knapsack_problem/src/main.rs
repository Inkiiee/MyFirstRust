use std::io;

struct Goods{
    val : i32,
    weight : i32
}

fn knapsack_problem(vgoods : &Vec<Goods>, c : u32) -> (i32,i32,i32) {
    let goods_num = vgoods.len() as usize;
    let mut values : Vec<Vec<i32>> = Vec::new();
    let mut pre : Vec<Vec<usize>> = Vec::new();
    let cap = c as usize;

    for _i in 0..goods_num+1 {
        let v : Vec<i32> = vec![0;cap+1];
        let p : Vec<usize> = vec![0;cap+1];
        values.push(v);
        pre.push(p);
    }

    for v in 1..goods_num+1 {
        for w in 1..cap+1 {
            if (vgoods[v-1].weight as usize) > w {
                values[v][w] = values[v-1][w];
                pre[v][w] = pre[v-1][w];
            }
            else{
                let wei = w - (vgoods[v-1].weight as usize);
                if values[v-1][w] < (values[v-1][wei] + vgoods[v-1].val) {
                    values[v][w] = values[v-1][wei] + vgoods[v-1].val;
                    pre[v][w] = v;
                }
                else{
                    values[v][w] = values[v-1][w];
                    pre[v][w] = pre[v-1][w];
                }
            }
        }
    }
    
    let num_and_good = trace_state_knapsack(&pre, vgoods);
    (values[goods_num][cap], num_and_good.0, num_and_good.1)
}

fn main() {
    let mut goods_num : i32;
    loop{
        println!("------please input num of goods-----");
        goods_num = input_num();
        if goods_num > 10 { 
            println!("num of goods must smaller than 10");
            continue; 
        }
        break;
    }

    let mut cap : u32;
    loop{
        println!("-----please input capacity of knapsack----");
        cap = input_num() as u32;
        if cap < 50 || cap > 100 { 
            println!("capacity of knapsack must be among 50 ~ 100");
            continue; 
        }
        break;
    }

    let mut vgoods : Vec<Goods> = Vec::new();
    for n in 0..goods_num {
        println!("-----please input {}th goods value-----", n+1);
        let val = input_num();
        println!("-----please input {}th goods weight------", n+1);
        let wei = input_num();

        let good = Goods{
            val : val,
            weight : wei
        };

        vgoods.push(good);
    }

    for i in 0..vgoods.len() {
        println!("{}th good value: {}, weight: {}", i+1, 
                 vgoods[i].val, vgoods[i].weight);
    }

    let val_num_wei = knapsack_problem(&vgoods, cap);
    println!("\nmax value: {}, weight: {}, num: {}, capacity: {}",
             val_num_wei.0, val_num_wei.2, val_num_wei.1, cap);
}

fn trace_state_knapsack(pre: &Vec<Vec<usize>>, vgoods: &Vec<Goods>) ->
                                                        (i32, i32) {
    let mut goods_num = pre.len() -1;
    let mut cap_num = pre[0].len() -1;
    let mut weight : i32 = 0;
    let mut num : i32 = 0;

    println!("\n-------list of goods---------");
    loop {
        let p = pre[goods_num][cap_num];
        if p == 0 { break; }

        weight += vgoods[p-1].weight;
        num+=1;
        
       println!("{}th goods in -> weight: {}, value: {}", p, 
                                vgoods[p-1].weight, vgoods[p-1].val);

        goods_num = p;
        cap_num -= vgoods[p-1].weight as usize;
    }

    (num, weight)
}

fn input_num() -> i32 {
    let num : i32;

    loop{
        let mut str_num : String = String::new();
        io::stdin().read_line(&mut str_num).expect("failed read");
        num = match str_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input number");
                continue;
            },
        };

        break;
    }

    num
}

