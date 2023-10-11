fn main() {
    store::Region::NorthEast()
    store::northeast::City::Albany
    for region in store::Region::regions() {
        for city in region.cities() {
            println!("{:?}", city)
        }
    }
}
