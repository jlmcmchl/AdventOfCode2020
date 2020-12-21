use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Mul,
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{
    storage::Storage, ArrayStorage, Dim, DimName, Matrix, MatrixN, MatrixSlice, Scalar, U1, U10,
    U96, U24, U8,
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

#[derive(Clone, Copy, Debug)]
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

impl Utilities<Pixel, U24, ArrayStorage<Pixel, U24, U24>> for MatrixN<Pixel, U24> {
    fn first_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U24,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::RStride,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::CStride,
    > {
        self.row(0)
    }

    fn last_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U24,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::RStride,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::CStride,
    > {
        self.row(23)
    }

    fn first_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U24,
        U1,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::RStride,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::CStride,
    > {
        self.column(0)
    }

    fn last_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U24,
        U1,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::RStride,
        <ArrayStorage<Pixel, U24, U24> as Storage<Pixel, U24, U24>>::CStride,
    > {
        self.column(23)
    }

    fn mirror_horizontal(&self) -> Self {
        Matrix::<_, U24, U24, _>::from_fn(|i, j| self[(i, 23 - j)])
    }

    fn mirror_vertical(&self) -> Self {
        Matrix::<_, U24, U24, _>::from_fn(|i, j| self[(23 - i, j)])
    }

    fn rotate_clockwise(&self) -> Self {
        Matrix::<_, U24, U24, _>::from_fn(|i, j| self[(23 - j, i)])
    }

    fn rotate_counterclockwise(&self) -> Self {
        Matrix::<_, U24, U24, _>::from_fn(|i, j| self[(j, 23 - i)])
    }

    fn to_orientation(&self, orient: Orientation) -> Self
    where
        Self: Sized,
    {
        match orient {
            Orientation::Normal0 => Matrix::<_, U24, U24, _>::from_fn(|i, j| self[(i, j)]),
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

impl Utilities<Pixel, U96, ArrayStorage<Pixel, U96, U96>> for MatrixN<Pixel, U96> {
    fn first_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U96,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::RStride,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::CStride,
    > {
        self.row(0)
    }

    fn last_row(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U1,
        U96,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::RStride,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::CStride,
    > {
        self.row(95)
    }

    fn first_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U96,
        U1,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::RStride,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::CStride,
    > {
        self.column(0)
    }

    fn last_column(
        &self,
    ) -> MatrixSlice<
        Pixel,
        U96,
        U1,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::RStride,
        <ArrayStorage<Pixel, U96, U96> as Storage<Pixel, U96, U96>>::CStride,
    > {
        self.column(95)
    }

    fn mirror_horizontal(&self) -> Self {
        Matrix::<_, U96, U96, _>::from_fn(|i, j| self[(i, 95 - j)])
    }

    fn mirror_vertical(&self) -> Self {
        Matrix::<_, U96, U96, _>::from_fn(|i, j| self[(95 - i, j)])
    }

    fn rotate_clockwise(&self) -> Self {
        Matrix::<_, U96, U96, _>::from_fn(|i, j| self[(95 - j, i)])
    }

    fn rotate_counterclockwise(&self) -> Self {
        Matrix::<_, U96, U96, _>::from_fn(|i, j| self[(j, 95 - i)])
    }

    fn to_orientation(&self, orient: Orientation) -> Self
    where
        Self: Sized,
    {
        match orient {
            Orientation::Normal0 => Matrix::<_, U96, U96, _>::from_fn(|i, j| self[(i, j)]),
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
        .filter(|(_, tiles)| tiles.len() == 2)
        .map(|(ind, tiles)| (*ind, tiles.clone()))
        .collect()
}

fn orient(
    tile: &MatrixN<Pixel, U10>,
    north: Option<usize>,
    west: Option<usize>,
    connections: &HashMap<usize, HashSet<usize>>,
) -> MatrixN<Pixel, U10> {
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
    .filter_map(|orient| {
        let tile = tile.to_orientation(*orient);
        let n_check = as_number(&tile.first_row());
        let w_check = as_number(&tile.first_column());

        if ((north.is_none() && !connections.contains_key(&n_check)) || north == Some(n_check))
            && ((west.is_none() && !connections.contains_key(&w_check)) || west == Some(w_check))
        {
            Some(tile)
        } else {
            None
        }
    })
    .next()
    .unwrap()
}

fn get_north(
    (row, col): (usize, usize),
    tiles: &Vec<(usize, MatrixN<Pixel, U10>)>,
) -> Option<usize> {
    if row > 0 {
        let rowlen = (tiles.len() - col) / row;
        let ind = row * rowlen + col - rowlen;
        // println!("\t{}/{} -> {}", row, col, ind);

        let (_, tile) = tiles[ind];

        Some(as_number(&tile.last_row()))
    } else {
        None
    }
}

fn get_west(
    (row, col): (usize, usize),
    tiles: &Vec<(usize, MatrixN<Pixel, U10>)>,
) -> Option<usize> {
    if col > 0 {
        let ind = if row > 0 {
            let rowlen = (tiles.len() - col) / row;
            row * rowlen + col - 1
        } else {
            col - 1
        };

        // println!("\t{}/{} -> {}", row, col, ind);
        let (_, tile) = tiles[ind];

        Some(as_number(&tile.last_column()))
    } else {
        None
    }
}

fn stitch<D: Dim + DimName>(tiles: &HashMap<usize, MatrixN<Pixel, U10>>) -> MatrixN<Pixel, D>
where
    <D as DimName>::Value: Mul,
    <<D as DimName>::Value as Mul>::Output: generic_array::ArrayLength<Pixel>,
    <<D as DimName>::Value as Mul>::Output: generic_array::ArrayLength<usize>,
{
    let mut used = HashSet::new();
    let connections = connections(tiles);
    let mut target = Matrix::<_, D, D, _>::from_fn(|_, _| Pixel::Off);

    let mut ids = Vec::new();

    // println!("{:?}", connections);

    let count = tiles.len();
    for col in (0..).take_while(|x| x * x < count) {
        for row in (0..).take_while(|x| x * x < count) {
            // everything else
            let north = get_north((col, row), &ids);
            let west = get_west((col, row), &ids);
            // println!("{}/{} -> {:?} {:?}", col, row, north, west);

            let (id, tile) = if let Some(connection) = north.or(west) {
                let id = connections[&connection]
                    .iter()
                    .filter(|e| !used.contains(*e))
                    .next()
                    .unwrap();

                let tile = orient(&tiles[&id], north, west, &connections);

                (id, tile)
            } else {
                // first tile
                tiles
                    .iter()
                    .filter(|(id, _)| {
                        connections
                            .iter()
                            .filter(|(_, set)| set.contains(id) && set.len() == 2)
                            .count()
                            == 4
                    })
                    .map(|(id, tile)| (id, orient(tile, None, None, &connections)))
                    .next()
                    .unwrap()
            };

            // println!("\t{}", id);

            ids.push((*id, tile));

            let tile = tile.fixed_slice::<U8, U8>(1, 1);

            target
                .fixed_slice_mut::<U8, U8>(col * 8, row * 8)
                .iter_mut()
                .enumerate()
                .for_each(|(ind, val)| *val = tile[ind]);

            used.insert(*id);
        }
    }

    // ids.iter().for_each(|(id, tile)| println!("{}:{}", id, tile));

    target
}

/* Sea Monster:
0123456789ABCDEFGHIJ
..................#.
#....##....##....###
.#..#..#..#..#..#...
 */

// TODO: This is the last required step to fix my solution.
// Then replace the necessary U24s to U96s and once it compiles we got it
fn count_serpents<D: DimName>(picture: &MatrixN<Pixel, D>) -> usize
where
    <D as DimName>::Value: Mul,
    <<D as DimName>::Value as Mul>::Output: generic_array::ArrayLength<Pixel>,
    <<D as DimName>::Value as Mul>::Output: generic_array::ArrayLength<usize>, 
{
    let (rows, cols) = picture.shape();

    let mut count = 0;

    for j in 0..rows - 2 {
        for i in 0..cols - 19 {
            // println!("{} {} {} {}", i, j, cols, rows);

            if picture.get((i + 18, j)) == Some(&Pixel::On)
                && picture.get((i + 0, j + 1)) == Some(&Pixel::On)
                && picture.get((i + 1, j + 2)) == Some(&Pixel::On)
                && picture.get((i + 4, j + 2)) == Some(&Pixel::On)
                && picture.get((i + 5, j + 1)) == Some(&Pixel::On)
                && picture.get((i + 6, j + 1)) == Some(&Pixel::On)
                && picture.get((i + 7, j + 2)) == Some(&Pixel::On)
                && picture.get((i + 10, j + 2)) == Some(&Pixel::On)
                && picture.get((i + 11, j + 1)) == Some(&Pixel::On)
                && picture.get((i + 12, j + 1)) == Some(&Pixel::On)
                && picture.get((i + 13, j + 2)) == Some(&Pixel::On)
                && picture.get((i + 16, j + 2)) == Some(&Pixel::On)
                && picture.get((i + 17, j + 1)) == Some(&Pixel::On)
                && picture.get((i + 18, j + 1)) == Some(&Pixel::On)
                && picture.get((i + 19, j + 1)) == Some(&Pixel::On)
            {
                // println!("\tFOUND");
                count += 1;
            }
        }
    }

    count
}

fn serpent_count(picture: &MatrixN<Pixel, U96>) -> usize
//where
//    <D as DimName>::Value: Mul,
//    <<D as DimName>::Value as Mul>::Output: generic_array::ArrayLength<Pixel>,
//    <<D as DimName>::Value as Mul>::Output: generic_array::ArrayLength<usize>,
{
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
    .filter_map(|orient| {
        let picture = picture.to_orientation(*orient);

        let count = count_serpents(&picture);

        // println!("{:?} -> {}:{}", orient, count, picture);

        if count > 0 {
            Some(count)
        } else {
            None
        }
    })
    .max()
    .unwrap()
}

#[aoc(day20, part2)]
pub fn solve_p2(tiles: &HashMap<usize, MatrixN<Pixel, U10>>) -> usize {
    let picture = stitch::<U96>(tiles);

    // println!("{}", picture);

    picture
        .iter()
        .map(|pixel| usize::from(*pixel))
        .sum::<usize>()
        - serpent_count(&picture) * 15
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
