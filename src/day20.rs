use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Mul,
    str::FromStr,
    unimplemented,
};

use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{
    storage::Storage, ArrayStorage, Dim, DimName, Matrix, MatrixN, MatrixSlice, Scalar, U1, U10,
    U109, U28,
};
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::iterator,
    IResult,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pixel {
    On,
    Off,
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::Off
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::On => f.write_str("#"),
            Pixel::Off => f.write_str("."),
        }
    }
}

impl From<Pixel> for usize {
    fn from(pix: Pixel) -> Self {
        match pix {
            Pixel::On => 1,
            Pixel::Off => 0,
        }
    }
}

impl FromStr for Pixel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            Err(format!("'{}' is more than 1 character", s))
        } else {
            match s {
                "#" => Ok(Pixel::On),
                "." => Ok(Pixel::Off),
                _ => Err(format!("'{}' is not a valid Pixel", s)),
            }
        }
    }
}

#[derive(Clone)]
enum Orientation {
    Normal0,
    Normal90,
    Normal180,
    Normal270,
    Flipped0,
    Flipped90,
    Flipped180,
    Flipped270,
}

trait Utilities<N: Scalar, D: Dim, S: Storage<N, D, D>> {
    fn first_row(&self) -> MatrixSlice<N, U1, D, S::RStride, S::CStride>;
    fn last_row(&self) -> MatrixSlice<N, U1, D, S::RStride, S::CStride>;

    fn first_column(&self) -> MatrixSlice<N, D, U1, S::RStride, S::CStride>;
    fn last_column(&self) -> MatrixSlice<N, D, U1, S::RStride, S::CStride>;

    fn mirror_horizontal(&self) -> Self;
    fn mirror_vertical(&self) -> Self;
    fn rotate_clockwise(&self) -> Self;
    fn rotate_counterclockwise(&self) -> Self;

    fn to_orientation(&self, orient: Orientation) -> Self
    where
        Self: Sized;
}

impl Utilities<Pixel, U10, ArrayStorage<Pixel, U10, U10>> for MatrixN<Pixel, U10> {
    fn first_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U10,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::RStride,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::CStride,
    > {
        self.row(0)
    }

    fn last_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U10,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::RStride,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::CStride,
    > {
        self.row(9)
    }

    fn first_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U10,
        U1,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::RStride,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::CStride,
    > {
        self.column(0)
    }

    fn last_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U10,
        U1,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::RStride,
        <ArrayStorage<Pixel, U10, U10> as Storage<Pixel, U10, U10>>::CStride,
    > {
        self.column(9)
    }

    fn mirror_horizontal(&self) -> Self {
        Matrix::<_, U10, U10, _>::from_fn(|i, j| self[(i, 9 - j)])
    }

    fn mirror_vertical(&self) -> Self {
        Matrix::<_, U10, U10, _>::from_fn(|i, j| self[(9 - i, j)])
    }

    fn rotate_clockwise(&self) -> Self {
        Matrix::<_, U10, U10, _>::from_fn(|i, j| self[(9 - j, i)])
    }

    fn rotate_counterclockwise(&self) -> Self {
        Matrix::<_, U10, U10, _>::from_fn(|i, j| self[(j, 9 - i)])
    }

    fn to_orientation(&self, orient: Orientation) -> Self
    where
        Self: Sized,
    {
        match orient {
            Orientation::Normal0 => Matrix::<_, U10, U10, _>::from_fn(|i, j| self[(i, j)]),
            Orientation::Normal90 => self.rotate_clockwise(),
            Orientation::Normal180 => self.mirror_vertical(),
            Orientation::Normal270 => self.rotate_counterclockwise(),
            Orientation::Flipped0 => self.mirror_horizontal(),
            Orientation::Flipped90 => self.mirror_horizontal().rotate_clockwise(),
            Orientation::Flipped180 => self.mirror_horizontal().mirror_vertical(),
            Orientation::Flipped270 => self.mirror_horizontal().rotate_counterclockwise(),
        }
    }
}

impl Utilities<Pixel, U28, ArrayStorage<Pixel, U28, U28>> for MatrixN<Pixel, U28> {
    fn first_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U28,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::RStride,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::CStride,
    > {
        self.row(0)
    }

    fn last_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U28,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::RStride,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::CStride,
    > {
        self.row(27)
    }

    fn first_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U28,
        U1,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::RStride,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::CStride,
    > {
        self.column(0)
    }

    fn last_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U28,
        U1,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::RStride,
        <ArrayStorage<Pixel, U28, U28> as Storage<Pixel, U28, U28>>::CStride,
    > {
        self.column(27)
    }

    fn mirror_horizontal(&self) -> Self {
        Matrix::<_, U28, U28, _>::from_fn(|i, j| self[(i, 27 - j)])
    }

    fn mirror_vertical(&self) -> Self {
        Matrix::<_, U28, U28, _>::from_fn(|i, j| self[(27 - i, j)])
    }

    fn rotate_clockwise(&self) -> Self {
        Matrix::<_, U28, U28, _>::from_fn(|i, j| self[(27 - j, i)])
    }

    fn rotate_counterclockwise(&self) -> Self {
        Matrix::<_, U28, U28, _>::from_fn(|i, j| self[(j, 27 - i)])
    }

    fn to_orientation(&self, orient: Orientation) -> Self
    where
        Self: Sized,
    {
        match orient {
            Orientation::Normal0 => Matrix::<_, U28, U28, _>::from_fn(|i, j| self[(i, j)]),
            Orientation::Normal90 => self.rotate_clockwise(),
            Orientation::Normal180 => self.mirror_vertical(),
            Orientation::Normal270 => self.rotate_counterclockwise(),
            Orientation::Flipped0 => self.mirror_horizontal(),
            Orientation::Flipped90 => self.mirror_horizontal().rotate_clockwise(),
            Orientation::Flipped180 => self.mirror_horizontal().mirror_vertical(),
            Orientation::Flipped270 => self.mirror_horizontal().rotate_counterclockwise(),
        }
    }
}

impl Utilities<Pixel, U109, ArrayStorage<Pixel, U109, U109>> for MatrixN<Pixel, U109> {
    fn first_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U109,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::RStride,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::CStride,
    > {
        self.row(0)
    }

    fn last_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U109,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::RStride,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::CStride,
    > {
        self.row(108)
    }

    fn first_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U109,
        U1,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::RStride,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::CStride,
    > {
        self.column(0)
    }

    fn last_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U109,
        U1,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::RStride,
        <ArrayStorage<Pixel, U109, U109> as Storage<Pixel, U109, U109>>::CStride,
    > {
        self.column(108)
    }

    fn mirror_horizontal(&self) -> Self {
        Matrix::<_, U109, U109, _>::from_fn(|i, j| self[(i, 108 - j)])
    }

    fn mirror_vertical(&self) -> Self {
        Matrix::<_, U109, U109, _>::from_fn(|i, j| self[(108 - i, j)])
    }

    fn rotate_clockwise(&self) -> Self {
        Matrix::<_, U109, U109, _>::from_fn(|i, j| self[(108 - j, i)])
    }

    fn rotate_counterclockwise(&self) -> Self {
        Matrix::<_, U109, U109, _>::from_fn(|i, j| self[(j, 108 - i)])
    }

    fn to_orientation(&self, orient: Orientation) -> Self
    where
        Self: Sized,
    {
        match orient {
            Orientation::Normal0 => Matrix::<_, U109, U109, _>::from_fn(|i, j| self[(i, j)]),
            Orientation::Normal90 => self.rotate_clockwise(),
            Orientation::Normal180 => self.mirror_vertical(),
            Orientation::Normal270 => self.rotate_counterclockwise(),
            Orientation::Flipped0 => self.mirror_horizontal(),
            Orientation::Flipped90 => self.mirror_horizontal().rotate_clockwise(),
            Orientation::Flipped180 => self.mirror_horizontal().mirror_vertical(),
            Orientation::Flipped270 => self.mirror_horizontal().rotate_counterclockwise(),
        }
    }
}

fn parse_tile(input: &str) -> IResult<&str, (usize, MatrixN<Pixel, U10>)> {
    let (input, _) = tag("Tile ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(":\n")(input)?;
    let mut pixels_iter = iterator(input, take(1usize));
    let pixels: Vec<Pixel> = pixels_iter.filter_map(|c: &str| c.parse().ok()).collect();

    let pixels_matrix = Matrix::<_, U10, U10, _>::from_row_slice(&pixels);
    let res = pixels_iter.finish();

    res.map(|(input, _)| (input, (id.parse().unwrap(), pixels_matrix)))
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> HashMap<usize, MatrixN<Pixel, U10>> {
    input
        .split("\n\n")
        .map(|tile| parse_tile(tile).unwrap().1)
        .collect()
}

fn as_number<R: Dim, C: Dim, RStride: Dim, CStride: Dim>(
    tile: &MatrixSlice<Pixel, R, C, RStride, CStride>,
) -> usize {
    tile.iter()
        .fold(0, |acc, pixel| acc << 1 | usize::from(*pixel))
}

#[aoc(day20, part1)]
pub fn solve_p1(tiles: &HashMap<usize, MatrixN<Pixel, U10>>) -> usize {
    let connections = tiles.iter().fold(
        HashMap::<usize, HashSet<usize>>::new(),
        |mut map, (id, tile)| {
            vec![
                Orientation::Normal0,
                Orientation::Normal90,
                Orientation::Normal180,
                Orientation::Normal270,
                Orientation::Flipped0,
                Orientation::Flipped90,
                Orientation::Flipped180,
                Orientation::Flipped270,
            ]
            .iter()
            .for_each(|orient| {
                let tile = tile.to_orientation(orient.clone());
                map.entry(as_number(&tile.first_row()))
                    .or_insert(HashSet::new())
                    .insert(*id);
            });
            map
        },
    );

    tiles
        .iter()
        .filter(|(id, _)| {
            connections
                .iter()
                .filter(|(_, set)| set.contains(id) && set.len() == 2)
                .count()
                == 4
        })
        .map(|(id, _)| id)
        .product()
}

fn connections(tiles: &HashMap<usize, MatrixN<Pixel, U10>>) -> HashMap<usize, HashSet<usize>> {
    tiles
        .iter()
        .fold(
            HashMap::<usize, HashSet<usize>>::new(),
            |mut map, (id, tile)| {
                vec![
                    Orientation::Normal0,
                    Orientation::Normal90,
                    Orientation::Normal180,
                    Orientation::Normal270,
                    Orientation::Flipped0,
                    Orientation::Flipped90,
                    Orientation::Flipped180,
                    Orientation::Flipped270,
                ]
                .iter()
                .for_each(|orient| {
                    let tile = tile.to_orientation(orient.clone());
                    map.entry(as_number(&tile.first_row()))
                        .or_insert(HashSet::new())
                        .insert(*id);
                });
                map
            },
        )
        .iter()
        .filter(|(id, tiles)| tiles.len() == 2)
        .map(|(ind, tiles)| (*ind, tiles.clone()))
        .collect()
}

fn orient(
    tile: &MatrixN<Pixel, U10>,
    north: Option<usize>,
    west: Option<usize>,
    connections: &HashMap<usize, HashSet<usize>>,
) -> MatrixN<Pixel, U10> {
    println!("{}", tile);
    match north {
        Some(north) => unimplemented!(),
        None => match west {
            Some(west) => {
                // Upper Left corner
                // Figure out which sides don't have any connections
                let tn = as_number(&tile.first_row());
                let tw = as_number(&tile.first_column());

                // check north
                if connections.contains_key(&tn) {
                    // north exists, use south
                    if connections.contains_key(&tw) {
                        // west exists, use south, east
                        println!("south, east");

                        if tw == west {
                            println!("fuck");
                            tile.to_orientation(Orientation::Normal180)
                        } else {
                            println!("fuck2");
                            tile.to_orientation(Orientation::Flipped0)
                        }
                    } else {
                        // use south, west
                        println!("south, west");

                        if tw == west {
                            println!("fuck");
                            tile.to_orientation(Orientation::Normal90)
                        } else {
                            println!("fuck2");
                            tile.to_orientation(Orientation::Flipped90)
                        }
                    }
                } else {
                    // use north
                    if connections.contains_key(&tw) {
                        // west exists, use north, east
                        println!("north, east");
                        if tw == west {
                            println!("fuck");
                            tile.to_orientation(Orientation::Normal270)
                        } else {
                            println!("fuck2");
                            tile.to_orientation(Orientation::Flipped270)
                        }
                    } else {
                        // it's correct
                        println!("north, west");

                        if tw == west {
                            println!("fuck");
                            tile.to_orientation(Orientation::Normal0)
                        } else {
                            println!("fuck2");
                            tile.to_orientation(Orientation::Flipped180)
                        }
                    }
                }
            }
            None => {
                // Upper Left corner
                // Figure out which sides don't have any connections
                let tn = as_number(&tile.first_row());
                let tw = as_number(&tile.first_column());

                // check north
                if connections.contains_key(&tn) {
                    // north exists, use south
                    if connections.contains_key(&tw) {
                        // west exists, use south, east
                        println!("south, east");

                        tile.to_orientation(Orientation::Normal180)
                    } else {
                        // use south, west
                        println!("south, west");

                        tile.to_orientation(Orientation::Normal90)
                    }
                } else {
                    // use north
                    if connections.contains_key(&tw) {
                        // west exists, use north, east
                        println!("north, east");

                        tile.to_orientation(Orientation::Normal270)
                    } else {
                        // it's correct
                        println!("north, west");

                        tile.to_orientation(Orientation::Normal0)
                    }
                }
            }
        },
    }
}

fn stitch<D: Dim + DimName>(tiles: &HashMap<usize, MatrixN<Pixel, U10>>) -> MatrixN<Pixel, D>
where
    <D as DimName>::Value: Mul,
    <<D as DimName>::Value as Mul>::Output: generic_array::ArrayLength<Pixel>,
{
    let mut used = HashSet::new();
    let mut connections = connections(tiles);
    let mut target = Matrix::<_, D, D, _>::from_fn(|_, _| Pixel::Off);

    let (id, tile) = tiles
        .iter()
        .filter(|(id, _)| {
            connections
                .iter()
                .filter(|(_, set)| set.contains(id) && set.len() == 2)
                .count()
                == 4
        })
        .next()
        .unwrap();

    let tile = orient(tile, None, None, &connections);

    used.insert(*id);
    target
        .slice_mut((0, 0), (10, 10))
        .iter_mut()
        .enumerate()
        .for_each(|(ind, pix)| *pix = tile[ind]);

    let west = as_number(&target.slice((0, 9), (10, 1)));
    let id = connections[&west].difference(&used).next().unwrap();
    let tile = tiles[&id];

    let tile = orient(&tile, None, Some(west), &connections);
    used.insert(*id);
    target
        .slice_mut((0, 9), (10, 10))
        .iter_mut()
        .enumerate()
        .for_each(|(ind, pix)| *pix = tile[ind]);

    println!("{:?}", used);

    target
}

#[aoc(day20, part2)]
pub fn solve_p2(tiles: &HashMap<usize, MatrixN<Pixel, U10>>) -> usize {
    let connections = connections(tiles);

    let picture = stitch::<U28>(tiles);

    println!("{}", picture);

    tiles.iter().for_each(|(id, _)| {
        println!(
            "{}: {}",
            id,
            connections
                .iter()
                .filter(|(_, set)| set.contains(id) && set.len() == 2)
                .count()
                / 2
        )
    });

    tiles.iter().count()
}

#[cfg(test)]
mod test {
    use super::*;

    fn p1_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p1(&parsed)
    }

    fn p2_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p2(&parsed)
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1_wrapper("Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###..."), 20899048083289);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper("Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###..."), 273);
    }
}
