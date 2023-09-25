fn sum_array(array: &mut[i32] ) -> i32{
    let mut sum=0;
    for i in array.iter() { //Recorro array sin modificarlo
        sum+=i;   
    }
    for i in array.iter_mut(){ //Recorro array y modifico
        *i+=sum;
    }
    return sum;
}
fn main(){
    let hola:&str="¡Hola ";
    let mundo="Mundo!";
    println!("{}",hola.to_owned()+mundo);

    let mut array:[i32; 4]=[1, 2, 4, 5]; //Declaro array de tamaño 4 de integers de 32 bits     
    let res = sum_array(&mut array); //Le paso la referencia del array?
    for elem in array{
        println!("{}", elem);
    }
    println!("{}", res);
}


