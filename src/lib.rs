use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

fn is_red(i: i32) -> bool{
    if i == 1 || i == 3 || i == 5 || i == 7 || i == 9 ||
       i == 12 || i == 14 || i == 16 || i == 18 ||
       i == 19 || i == 21 || i == 23 || i == 25 || i == 27 ||
       i == 30 || i == 32 || i == 34 || i == 36
    {
        return true;
    }
    return false;
}

#[wasm_bindgen]
pub fn compute_triggers(min_succession:i32, inputs: Vec<i32>) -> Vec<JsValue> {
    let mut ret: Vec<JsValue> = vec![];

    //inputs[0] is the last number appeared

    let mut count_1_18 = true;
    let mut nb_succession_1_18 = 0;

    let mut count_19_36 = true;
    let mut nb_succession_19_36 = 0;

    let mut count_red = true;
    let mut nb_succession_red = 0;

    let mut count_black = true;
    let mut nb_succession_black = 0;
    
    let mut count_pair = true;
    let mut nb_succession_pair = 0;

    let mut count_impair = true;
    let mut nb_succession_impair = 0;
    

    for i in inputs{
        if count_1_18 && 1 <= i && i <= 18{
            nb_succession_1_18 += 1;
        }else{
            count_1_18 = false;
        }

        if count_19_36 && 19 <= i && i <= 36{
            nb_succession_19_36 += 1;
        }else{
            count_19_36 = false;
        }

        if count_red && is_red(i){
            nb_succession_red += 1;
        }else{
            count_red = false;
        }

        if count_black && !is_red(i) && i!=0{
            nb_succession_black += 1;
        }else{
            count_black = false;
        }

        if count_pair && i%2==0 && i!=0{
            nb_succession_pair += 1;
        }else{
            count_pair = false;
        }

        if count_impair && i%2==1{
            nb_succession_impair += 1;
        }else{
            count_impair = false;
        }
    }

    if nb_succession_1_18 >= min_succession{
        ret.push(JsValue::from(format!("succession of 1-18: {}, consider playing 19-36", nb_succession_1_18)));
    }
    if nb_succession_19_36 >= min_succession{
        ret.push(JsValue::from(format!("succession of 19-36: {}, consider playing 1-18", nb_succession_19_36)));
    }
    if nb_succession_red >= min_succession{
        ret.push(JsValue::from(format!("succession of red: {}, consider playing black", nb_succession_red)));
    }
    if nb_succession_black >= min_succession{
        ret.push(JsValue::from(format!("succession of black: {}, consider playing red", nb_succession_black)));
    }
    if nb_succession_pair >= min_succession{
        ret.push(JsValue::from(format!("succession of pair: {}, consider playing impair", nb_succession_pair)));
    }
    if nb_succession_impair >= min_succession{
        ret.push(JsValue::from(format!("succession of pair: {}, consider playing pair", nb_succession_impair)));
    }
    return ret;

}