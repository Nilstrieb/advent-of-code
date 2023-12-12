fn line_match_count(line: &str) -> usize {
    let line = line.as_bytes();
    fn pack(chunk: &[u8]) -> u16 {
        ((chunk[1] as u16) << 8) | (chunk[2] as u16)
    }

    let mut numbers = line
        .split(|&b| b == b':')
        .nth(1)
        .unwrap()
        .split(|&b| b == b'|');
    let winning = numbers
        .next()
        .unwrap()
        // Chunks of double digit numbers with a leading space. We don't care about the trailing space.
        .chunks_exact(3)
        .map(pack)
        .collect::<arrayvec::ArrayVec<_, 16>>();

    let you_have = numbers.next().unwrap().chunks_exact(3).map(pack);

    you_have
        .filter(|have| winning.iter().any(|w| w == have))
        .count()
}

pub fn part2(input: &str) -> u64 {
    let lines = input.lines().map(line_match_count).collect::<Vec<_>>();

    let mut cache = vec![0; lines.len()];

    let mut processed = 0;

    // By iterating backwards, we ensure the cache is always populated for every subsequent line.
    for (i, result) in lines.iter().copied().enumerate().rev() {
        let before = processed;
        processed += 1; // Every card gives us one card.
                        // Now, let's see how many cards this card will expand to.
        for expand in (i + 1)..((i + 1) + result) {
            #[cfg(debug_assertions)]
            eprintln!(
                "{} expands to {} which is worth {}",
                i + 1,
                expand + 1,
                cache[expand]
            );
            // Since the value is bigger than i, it must be cached!
            processed += cache[expand];
        }
        cache[i] = processed - before;
    }

    processed
}
