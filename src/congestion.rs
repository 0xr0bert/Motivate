// Congestion for road space should be able to be represented with a supply / demand curve
//
//       demand         supply                   
// price | \              |                                               
//   of  |  \             |                                               
//  road |   \            |                                               
// space |    \           |                                               
//       |     \          |                                               
//       |      \         |                                               
//       |       \        |                                               
//       |        \       |                                               
//       |         \      |                                               
//       |          \     |                                               
//       |           \    |                                               
//       |            \   |                                               
//       |             \  |                                               
//       |              \ |                                               
//       |               \|                                               
//  Pe ->|                | <- Free market road space                                             
//       |                |\                                              
//       |                | \                                             
//       |                |  \                                            
//       |                |   \                                           
//       |                |    \                                          
//       |                |     \                                         
//       |                |      \                                        
//       |                |       \                                       
//       |                |        \                                      
//       |                |         \                                     
//       |                |          \                                    
//       |                |           \                                   
//       |                |            \                                  
//       |                |             \                                 
//       |                |              \                                
//       |                |               \                               
//  P0 ->+----------------|-----------------------------------------------
//                        ^               ^              Quantity of road space
//                        |               |
//                        Qe              Q0
//
// As the price of road space is free, this leads to a demand for road space at Q0, but only a supply at Qe,
// therefore there is a shortage in supply of road space, which is congestion.
//
// Neighbourhood.capacity is the supply curve, unsure of what a realistic demand curve would look like, other than
// it being price inelastic
//
// Congestion for public transport should follow the same graph, substuting road space for public transport space?