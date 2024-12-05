
// pub(crate) struct Histogram64<T> {
//     pub bins: [T; 64],
//     pub populations: [usize; 64],
// }

// impl<T> Histogram64<T> {

//     /// Returns maximal population found in this [Histogram]
//     pub fn max_population(&self) -> usize {
//         let mut max = 0;
//         for pop in &self.populations {
//             if *pop > max {
//                 max = *pop;
//             }
//         }
//         max
//     }

//     /// Returns bin quantization for this population
//     pub fn get_by_min_population(&self, population: &usize) -> Option<&T> {
//         for (index, pop) in self.populations.iter().enumerate() {
//             if index > 0 {
//                 if pop >= population {
//                     return self.bins.get(index);
//                 }
//             }
//         }
//         None
//     }
// }