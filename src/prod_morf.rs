pub fn traducao<A, B, C, D, F>(t1: (A, B), t2: (C, D), f: F) -> Option<(C, D)>
where
    F: Fn((A, B)) -> (C, D),
    C: std::fmt::Debug + PartialEq,
    D: std::fmt::Debug + PartialEq,
{
    let (c2, d2) = f(t1);

    if c2 != t2.0 {
        return None;
    }

    if d2 != t2.1 {
        return None;
    }

    Some((c2, d2))
}

#[cfg(test)]
mod test_random_morfism {
    use rand::{Rng, SeedableRng, rngs::StdRng};

    use super::*;

    fn pure_random(seed: i32) -> i32 {
        let mut rng = StdRng::seed_from_u64(seed as u64);

        rng.random()
    }

    fn identidade(c: char) -> char {
        c
    }

    #[test]
    fn teste_morfismo_com_seed() {
        let t1: (i32, char) = (42, 'z');

        let morfismo = |(n, c): (i32, char)| -> (i32, char) { (pure_random(n), identidade(c)) };

        let valor_esperado = pure_random(42);
        let t2: (i32, char) = (valor_esperado, 'z');

        let resultado = traducao(t1, t2, morfismo);

        assert_eq!(resultado, Some(t2));

        assert_eq!(morfismo((42, 'z')), t2);
    }
}
