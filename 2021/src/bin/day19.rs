use std::{collections::HashSet, fs};

type Coordinate = [i32; 3];

fn parse(data: &str) -> Vec<Vec<[i32; 3]>> {
    let beacons: Vec<Vec<_>> = data
        .split("\n\n")
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.splitn(3, ',').map(|n| n.parse::<i32>().unwrap());
                    [
                        nums.next().unwrap(),
                        nums.next().unwrap(),
                        nums.next().unwrap(),
                    ]
                })
                .collect()
        })
        .collect();
    beacons
}

fn count_overlap(dists: &Vec<usize>, dists2: &Vec<usize>) -> usize {
    let length = dists.len();
    let length2 = dists2.len();
    let mut i = 0;
    let mut i2 = 0;
    let mut count = 0;
    while i < length && i2 < length2 {
        if dists[i] == dists2[i2] {
            count += 1;
            i += 1;
            i2 += 1;
        } else if dists[i] < dists2[i2] {
            i += 1;
        } else if dists[i] > dists2[i2] {
            i2 += 1;
        }
    }
    count
}

fn calc_distance(cood1: &Coordinate, cood2: &Coordinate) -> usize {
    ((cood2[0] - cood1[0]).pow(2) + (cood2[1] - cood1[1]).pow(2) + (cood2[2] - cood1[2]).pow(2))
        as usize
}

#[derive(Debug, Clone, Copy)]
struct Transform {
    pos: usize,
    offset: i32,
    sign: i32,
}
fn extract_relative(
    cood0: &Coordinate,
    cood1: &Coordinate,
    cood0_source: &Coordinate,
    cood1_source: &Coordinate,
) -> [Transform; 3] {
    let diff_source = [
        cood0_source[0] - cood1_source[0],
        cood0_source[1] - cood1_source[1],
        cood0_source[2] - cood1_source[2],
    ];
    let diff_source_abs = diff_source.map(|n| n.abs());

    let diff_x = cood0[0] - cood1[0];
    let pos_x = diff_source_abs
        .iter()
        .position(|n| *n == diff_x.abs())
        .unwrap();

    let diff_y = cood0[1] - cood1[1];
    let pos_y = diff_source_abs
        .iter()
        .position(|n| *n == diff_y.abs())
        .unwrap();

    let diff_z = cood0[2] - cood1[2];
    let pos_z = diff_source_abs
        .iter()
        .position(|n| *n == diff_z.abs())
        .unwrap();

    let sign = [
        diff_x / diff_source[pos_x],
        diff_y / diff_source[pos_y],
        diff_z / diff_source[pos_z],
    ];
    [
        Transform {
            pos: pos_x,
            offset: cood0[0] * sign[0] - cood0_source[pos_x],
            sign: sign[0],
        },
        Transform {
            pos: pos_y,
            offset: cood0[1] * sign[1] - cood0_source[pos_y],
            sign: sign[1],
        },
        Transform {
            pos: pos_z,
            offset: cood0[2] * sign[2] - cood0_source[pos_z],
            sign: sign[2],
        },
    ]
}

fn find_common_scanner(
    scanner_idx: usize,
    scanner: &Vec<Vec<usize>>,
    distance: &Vec<Vec<Vec<usize>>>,
    beacons: &Vec<Vec<Coordinate>>,
) -> Vec<(usize, Vec<([i32; 3], usize)>)> {
    distance
        .iter()
        .enumerate()
        .filter_map(|(idx_other, dist_other)| {
            let common_beacons: Vec<_> = scanner
                .iter()
                .enumerate()
                .filter_map(|(cood_idx, dist)| {
                    let overlap_idx =
                        dist_other
                            .iter()
                            .enumerate()
                            .find_map(|(source_idx, dist_source)| {
                                if count_overlap(dist, dist_source) == 12 {
                                    Some(source_idx)
                                } else {
                                    None
                                }
                            });
                    if let Some(idx) = overlap_idx {
                        Some((beacons[scanner_idx][cood_idx], idx))
                    } else {
                        None
                    }
                })
                .collect();
            if common_beacons.len() == 12 {
                Some((idx_other, common_beacons))
            } else {
                None
            }
        })
        .collect()
}

fn combine_transform(
    transform: &[Transform; 3],
    source_transform: &[Transform; 3],
) -> [Transform; 3] {
    [
        Transform {
            pos: source_transform[transform[0].pos].pos,
            offset: source_transform[transform[0].pos].offset
                + transform[0].offset * source_transform[transform[0].pos].sign,
            sign: transform[0].sign * source_transform[transform[0].pos].sign,
        },
        Transform {
            pos: source_transform[transform[1].pos].pos,
            offset: source_transform[transform[1].pos].offset
                + transform[1].offset * source_transform[transform[1].pos].sign,
            sign: transform[1].sign * source_transform[transform[1].pos].sign,
        },
        Transform {
            pos: source_transform[transform[2].pos].pos,
            offset: source_transform[transform[2].pos].offset
                + transform[2].offset * source_transform[transform[2].pos].sign,
            sign: transform[2].sign * source_transform[transform[2].pos].sign,
        },
    ]
}

fn find_relative0_transform(beacons: &Vec<Vec<[i32; 3]>>) -> Vec<[Transform; 3]> {
    let distance: Vec<Vec<Vec<_>>> = beacons
        .iter()
        .map(|scanner| {
            scanner
                .iter()
                .map(|beacon| {
                    let mut dists: Vec<_> = scanner
                        .iter()
                        .map(|beacon2| calc_distance(&beacon, &beacon2))
                        .collect();
                    dists.sort();
                    dists
                })
                .collect()
        })
        .collect();
    let mut relative_offsets = distance.iter().enumerate().skip(1).fold(
        vec![vec![(
            0,
            [
                Transform {
                    pos: 0,
                    offset: 0,
                    sign: 1,
                },
                Transform {
                    pos: 1,
                    offset: 0,
                    sign: 1,
                },
                Transform {
                    pos: 2,
                    offset: 0,
                    sign: 1,
                },
            ],
        )]],
        |mut offsets, (scanner_idx, scanner)| {
            let common_scanners = find_common_scanner(scanner_idx, scanner, &distance, beacons);
            offsets.push(
                common_scanners
                    .iter()
                    .map(|common| {
                        let transofrm = extract_relative(
                            &common.1[0].0,
                            &common.1[1].0,
                            &beacons[common.0][common.1[0].1],
                            &beacons[common.0][common.1[1].1],
                        );
                        (common.0, transofrm)
                    })
                    .collect(),
            );
            offsets
        },
    );
    let mut relative0_transform: Vec<Option<_>>;
    loop {
        relative0_transform = relative_offsets
            .iter()
            .map(|scanner| {
                if scanner[0].0 == 0 {
                    Some(scanner[0])
                } else {
                    None
                }
            })
            .collect();
        if relative0_transform.iter().filter_map(|s| *s).count() == relative_offsets.len() {
            break;
        }
        relative_offsets = relative_offsets
            .into_iter()
            .map(|scanner| {
                let source0 = scanner.iter().find_map(|(source, transform)| {
                    if let Some((_, source_transform)) = relative0_transform[*source] {
                        Some((0, combine_transform(&transform, &source_transform)))
                    } else {
                        None
                    }
                });
                if let Some(source0) = source0 {
                    vec![source0]
                } else {
                    scanner
                }
            })
            .collect();
    }
    relative0_transform.iter().map(|o| o.unwrap().1).collect()
}

fn get_pos(transform: &[Transform; 3]) -> (usize, usize, usize) {
    (
        transform.iter().position(|t| t.pos == 0).unwrap(),
        transform.iter().position(|t| t.pos == 1).unwrap(),
        transform.iter().position(|t| t.pos == 2).unwrap(),
    )
}
fn calc(data: &str) -> usize {
    let beacons = parse(data);
    let relative0_transform = find_relative0_transform(&beacons);
    let trans_beacons: Vec<Vec<_>> = beacons
        .iter()
        .enumerate()
        .map(|(i, scanner)| {
            let transform = &relative0_transform[i];
            scanner
                .iter()
                .map(|cood| {
                    let pos = get_pos(&transform);
                    [
                        cood[pos.0] * transform[pos.0].sign - transform[pos.0].offset,
                        cood[pos.1] * transform[pos.1].sign - transform[pos.1].offset,
                        cood[pos.2] * transform[pos.2].sign - transform[pos.2].offset,
                    ]
                })
                .collect()
        })
        .collect();
    let beacons = trans_beacons.iter().fold(HashSet::new(), |mut set, coods| {
        coods.iter().for_each(|cood| {
            set.insert(*cood);
        });
        set
    });
    beacons.len()
}

fn calc2(data: &str) -> i32 {
    let beacons = parse(data);
    let relative0_transform = find_relative0_transform(&beacons);
    let scanner_cood: Vec<_> = relative0_transform
        .iter()
        .map(|transform| {
            let pos = get_pos(&transform);
            [
                transform[pos.0].offset * -1,
                transform[pos.1].offset * -1,
                transform[pos.2].offset * -1,
            ]
        })
        .collect();
    let mut max = 0;
    for cood1 in scanner_cood.iter() {
        for cood2 in scanner_cood.iter() {
            max = max.max(
                (cood1[0] - cood2[0]).abs()
                    + (cood1[1] - cood2[1]).abs()
                    + (cood1[2] - cood2[2]).abs(),
            );
        }
    }
    max
}

fn main() {
    let contents = fs::read_to_string("2021/input/day19.txt").unwrap();
    let n = calc(&contents);
    println!("Part1: {}", n);

    let n = calc2(&contents);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_overlap_test() {
        assert_eq!(
            12,
            count_overlap(
                &vec![
                    0, 2421, 8921, 803632, 881364, 970609, 1016552, 1431421, 1680306, 1690317,
                    1712390, 1724746, 1727350, 1743353, 1989710, 2052564, 2169041, 2218102,
                    2218857, 3420651, 3477902, 3752005, 4486238, 4593369, 5046257
                ],
                &vec![
                    0, 2421, 8921, 614153, 803632, 970609, 1016552, 1391869, 1431421, 1592339,
                    1680306, 1712390, 1743353, 1897534, 2141390, 2218102, 2218857, 2264961,
                    2316929, 2431441, 2583937, 2658365, 2905206, 3099005, 3099017
                ]
            )
        );
    }

    static DATA: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;

    #[test]
    fn part1_test() {
        assert_eq!(79, calc(&DATA));
    }

    #[test]
    fn part2_test() {
        assert_eq!(3621, calc2(&DATA));
    }
}
