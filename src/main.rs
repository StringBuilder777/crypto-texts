use std::collections::{HashMap};

fn main() {
    let text = "¡KXUBCBÑ KÑT PR HXLÑ!
EPOKRCG KÑT HXL0 BP JGXNCW RCW EPBUKUTCW BPR TUYÑ QCGC KCEIUCGRPW PR WCIÑG PW GUPWLÑWÑ. CRLXTÑW HXLÑW, KÑEÑ PR BP NÑGÑTHC (QÑEPRÑ), QXPBPT UTKGPEPTNCG RC QÑNPTKUC BP KUPGNCW EPBUKUTCW, PT PWQPKUCR RÑW CTNUIUÑNUKÑW D CTNAUWNCEUTUKÑW. GPWXRNCBÑ? XTC WÑIGPBÑWUW  ÑNGÑW HXLÑW KUNGUKÑW CEUTÑGCT RÑW PJPKNÑW KXGCNUZÑW, QÑG RÑ VXP PR HXLÑ ECW UTBUKCBÑ WPGUC PR BP ECTOCTCW. NCEQÑKÑ BPIPT EPOKRCGWP RCW EPBUKUTCW KÑT RC RPKAP, VXP GPCKKUÑTC ECR KÑT KUPGNÑW CTNUIUÑNUKÑW. ¡PT JUT! VXP RÑ ECW GPKÑEPTBCIRP PW ZÑRZPG CR KRCWUKÑ ZCWÑ BP CLXC.";

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

    let letras_espanol = [
    'E', 'A', 'O', 'I', 'N', 'S', 'D', 'R', 'U', 'L',
    'T', 'Ñ', 'P', 'M', 'B', 'G', 'V', 'F', 'Y', 'J',
    'Q', 'H', 'Z', 'X', 'W', 'K'
    ];

    let mut equi: Vec<(char, char)> = Vec::new();
   for ((letra_cifrada, _cantidad), letra_espanol) in
    frec.iter().zip(letras_espanol.iter())
    {
    equi.push((**letra_cifrada, *letra_espanol));
    }

    print!("\n\nlist \n {:?}",equi);

}