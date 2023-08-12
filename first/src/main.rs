use std::io;
const SECONDS_IN_MINUTE: u32 = 60;

fn main(){
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;
    let total = 30_u32;
    {
        let total = "quarenta";
        println!("Trabalhou {} horas",total);
    }
    println!("Trabalhou {} horas",total);

    let total_em_segundos = total * SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos", total_em_segundos);

    //tupla
    let mut numbers = (1,2,3.5);
    numbers.1 = 7;
    numbers = (3,4,5.5);
    let (_,_, c) = numbers;
    println!("{:?}", numbers);
    println!("{:?}", c);
    println!("{:?}", numbers.0);


    //array
    let mut numeros = [1, 2, 3, 4];
    numeros[0] = 0;
    println!("{:?}",numeros);
    println!("{:?}",numeros[1]);
    //slice
    println!("{:?}",&numeros[1..2]);
    println!("{:?}",&numeros[2..]);
    println!("{:?}",&numeros[..2]);

    //strings
    //char
    let l0 = 'd';
    let l1 = 'o';
    let l2 = 'u';
    let l3 = 'g';
    println!("{l0}{l1}{l2}{l3}");

    let nome: &str = "Douglas"; //&str tamanho conhecido
    println!("{nome}");

    let mut s = String::new();
    s.push_str("Douglas");
    s.push_str(" ");
    s.push_str("Poma");
    println!("{s}");

    // let st: String = "Douglas".to_string();
    // let st: String = "Douglas".into(); // require notation of type
    // let st: String = "Douglas".to_owned(); 
    let st = String::from("Douglas"); // Tamanho dinamico, usando push para alterar (String owned)
    println!("{st}");

    println!("{:-^20}", "Inputs");
/*     let mut text = String::new();
    println!("Digite um texto");
    io::stdin()
        .read_line(&mut text)
        .expect("Error reading console");
    
    println!("Você digitou: {text}");
    println!("Quantidade de letras {}", text.trim().len());
    println!("Quantidade de letras {}", text.trim().chars().count());
    println!("Maiusculas: {}", text.to_uppercase());
    println!("Minusculas: {}", text.to_lowercase());
    println!("Replace: {}", text.replace("a", "ba")); */
    println!("{}", "-".repeat(20));
    
    let input = "10";
    let input: u32 = input.parse().expect("Erro ao parsear");
    println!("Parseado {}", input+1);   
    
    println!("{:-^20}", "Functions");

    fn say_hello(name: &str){
        println!("Hello {name}");
    }
    say_hello("Douglas");
    fn somando(x: i32, y: i32) -> i32 {
        if x == 0 {
            return y;
        }
        // return x + y;
        x + y
    }
    let result = somando(3,4);
    println!("Resultado: {}", result);

    /* fn convert_to_number(s: &str) -> i32 {
        s.parse().unwrap()
    } */
    /* fn double(n: i32) -> i32{
        n * 2
    } */
    let input = "4 5 6 34 32 55";
    let result: Vec<i32> = input.split(' ')
        // .map(convert_to_number)
        .map(|s| s.parse::<i32>().unwrap())
        .map(|n| n * 2 )
        // .map(double)
        .collect();
    println!("{:?}",result);
    println!("{}", "-".repeat(20));
    println!("{:-^20}", "Ownership e Borrowing");
    //Memory Stack
    let a = 1; // Copy(i32, f64, bool, char)
    let b = a; // nova locação
    println!("O valor de A é {}", a);
    println!("O valor de B é {}", b);
    let b = &a; // referencia
    println!("O valor de A é {}", a);
    println!("O valor de B é {}", *b);
    //Memory Heap
    let a = String::from("Douglas"); // A é dono do texto #No Copy
    // let b = a; // move o valor de A para B e invalida o A, (A não é mais dono e sim o B)
    let b = &a; //usa a referencia
    println!("O valor de A é {}", a);
    println!("O valor de B é {}", b);

    println!("");

/*     fn print_hello(text: &str) { // o valor da variavel é copiado para ser usado pela função
        println!("Hello, {text}");
    } */
/*     fn print_hello(text: String) { // a variavel é movida para a função, deixando de existir no escopo anterior
        println!("Hello, {text}");
    } */
    fn print_hello(text: &String) { // a variavel é movida para a função, deixando de existir no escopo anterior
        println!("Hello, {text}");
    }
    fn print_goodbye(text: String) { // String na Heap
        println!("Goodbye, {text}");
    }
    let name = "Douglas".to_string();
    // print_hello(name); // para tipo variaveis na HEap a variável é movida (OWNERSHIP)
    // print_hello(name.clone()); // força a cópia e não perde o valor da original
    print_hello(&name); //passa apenas a referencia, não perde titularidade e não cria clone (BORROWING)
    print_goodbye(name);

    println!("");

    fn to_uppercase(text: &mut String) {
        *text = text.to_uppercase();
    }
    fn add_prefix(text: &mut String) {
        *text = format!("FOO_{text}");
        // text.push_str("_FOO"); // caso use alguma coisa apos o ponto
    }
    let mut name = "Douglas Poma".to_string();
    to_uppercase(&mut name); //mut borrow   
    add_prefix(&mut name);
    println!("Nome alterado {name}");    

    println!("{}", "-".repeat(20));
    println!("{:-^20}", "Struct");

    struct Carro {
        modelo: String,
        cor: String,
        valor: u32
    }
    impl Carro {
        fn valor_div_dez(&self) -> u32 {
            self.valor / 10
        }
    }
    let meu_carro = Carro{
        modelo: String::from("Tesla Model X"),
        cor: String::from("azul"),
        valor: 970_000,
    };
    println!("Meu carro é um {} {} e custou {} reais em 10 parcelas de {}", 
        meu_carro.modelo, 
        meu_carro.cor, 
        meu_carro.valor,
        meu_carro.valor_div_dez()
    );
    println!("{}", "-".repeat(20));
    println!("{:-^20}", "match(switch)");
    let idade = 99;
    match idade {
        10 => println!("Novo"),
        18 => println!("De maior"),
        60 => println!("Velho"),
        _ => println!("Velhão")
    }
    println!("{}", "-".repeat(20));
    println!("{:-^20}", "enum");
    enum Turno {
        Manha,
        Tarde,
        Noite,
    }
    let trabalho_turno: Turno = Turno::Manha;
    match trabalho_turno {
        Turno::Manha => println!("Matutino"),
        Turno::Tarde => println!("Vespertino"),
        Turno::Noite => println!("Noturno"),
    }

    println!("{}", "-".repeat(20));
    println!("{:-^20}", "match(switch)");

    let banner = "Fim da execução
        desenvolvido por:  Douglas Poma";
        
    println!("{banner}");
}