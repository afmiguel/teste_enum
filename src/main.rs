enum EstadoAnimal {
    Vivo,
    Extinto,
    Desconhecido,
}

fn imprimir_estado(estado: EstadoAnimal) {
    if let EstadoAnimal::Vivo = estado {
        println!("O animal está vivo");
    } else if let EstadoAnimal::Extinto = estado {
        println!("O animal está extinto");
    } else if let EstadoAnimal::Desconhecido = estado {
        println!("O estado do animal é desconhecido");
    }
}

fn main() {
    let estado1 = EstadoAnimal::Vivo;
    let estado2 = EstadoAnimal::Extinto;
    let estado3 = EstadoAnimal::Desconhecido;
    
    imprimir_estado(estado1);
    imprimir_estado(estado2);
    imprimir_estado(estado3);
}
