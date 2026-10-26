use std::collections::HashMap;

fn main() {
    let text = "¡KXUBCBÑ KÑT PR HXLÑ!
    EPOKRCG KÑT HXL0 BP JGXNCW RCW EPBUKUTCW BPR TUYÑ QCGC KCEIUCGRPW PR WCIÑG PW GUPWLÑWÑ. 
    CRLXTÑW HXLÑW, KÑEÑ PR BP NÑGÑTHC (QÑEPRÑ), QXPBPT UTKGPEPTNCG RC QÑNPTKUC BP KUPGNCW EPBUKUTCW, PT PWQPKUCR RÑW CTNUIUÑNUKÑW D CTNAUWNCEUTUKÑW. 
    GPWXRNCBÑ? XTC WÑIGPBÑWUW  ÑNGÑW HXLÑW KUNGUKÑW CEUTÑGCT RÑW PJPKNÑW KXGCNUZÑW, QÑG RÑ VXP PR HXLÑ ECW UTBUKCBÑ WPGUC PR BP ECTOCTCW. NCEQÑKÑ BPIPT EPOKRCGWP RCW EPBUKUTCW KÑT RC RPKAP, VXP GPCKKUÑTC ECR KÑT KUPGNÑW CTNUIUÑNUKÑW. 
    ¡PT JUT! VXP RÑ ECW GPKÑEPTBCIRP PW ZÑRZPG CR KRCWUKÑ ZCWÑ BP CLXC.";

    let mut map_text: HashMap<char, usize> = HashMap::new();

    for cha in text.chars() {
        if cha.is_alphabetic(){
            let con = map_text.entry(cha).or_insert(0);
            *con +=1;
        }
    }

    let mut frec: Vec<_> = map_text.iter().collect();
    frec.sort_by_key(|&(_, cant)| cant);
    frec.reverse();

    print!("list \n {:?}",frec);

    let letras_espanol_v1 = [
    'E', 'A', 'O', 'I', 'N', 'S', 'D', 'R', 'U', 'L',
    'T', 'Ñ', 'P', 'M', 'B', 'G', 'V', 'F', 'Y', 'J',
    'Q', 'H', 'Z', 'X', 'W', 'K'
    ];

    let letras_espanol_v2 = [
    'A', 'E', 'I', 'O', 'S', 'N', 'L', 'D', 'R', 'U',
    'Ñ', 'T', 'M', 'P', 'G', 'B', 'F', 'V', 'J', 'Y',
    'H', 'Q', 'X', 'Z', 'K', 'W'
    ];

    let mut equi: Vec<(char, char)> = Vec::new();
   for ((letra_cifrada, _cantidad), letra_espanol_v1) in frec.iter().zip(letras_espanol_v1.iter())
    {
    equi.push((**letra_cifrada, *letra_espanol_v1));
    }

    let mut equi_v2: Vec<(char, char)> = Vec::new();
   for ((letra_cifrada, _cantidad), letra_espanol_v2) in frec.iter().zip(letras_espanol_v2.iter())
    {
    equi_v2.push((**letra_cifrada, *letra_espanol_v2));
    }

    print!("\n\nlist \n {:?}",equi);
    print!("\n\nlist \n {:?}",equi_v2);

    println!("\n\nTexto descifrado V1:\n");

for caracter in text.chars() {
    if let Some((_, reemplazo)) = equi.iter().find(|(origen, _)| *origen == caracter) {
        print!("{reemplazo}");
    } else {
        print!("{caracter}");
    }
}

println!("\n\nTexto descifrado V2:\n");

for caracter in text.chars() {
    if let Some((_, reemplazo)) = equi_v2.iter().find(|(origen, _)| *origen == caracter) {
        print!("{reemplazo}");
    } else {
        print!("{caracter}");
    }
}

println!();
}
