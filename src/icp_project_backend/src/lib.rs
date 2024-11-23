use ic_cdk_macros::{init, query, update};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use candid::CandidType;
use candid::export_service;


#[derive(Clone, CandidType, Serialize, Deserialize)]
struct NFTData {
    id: u64,
    name: String,
    owner: String,
    creator: String,
    metadata: FileType,
    collections_id: u64,
}
impl Default for NFTData {
    fn default() -> Self {
        NFTData {
            id: 0,
            name: String::new(),
            owner: String::new(), // Default value for Principal
            creator: String::new(),
            metadata: FileType { file: Vec::new(),file_type:String::from("image/jpg") },
            collections_id: 0,
        }
    }
}
#[derive(Clone,CandidType, Serialize, Deserialize)]
struct NftListing {
    nft: NFTData,
    owner: String,
    price: u64,
    bids: BTreeMap<String, u64>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Brand {
    name: String,
    rented: Vec<NFTData>,
    products: Vec<Product>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Creator {
    name: String,
    collections: Vec<NftCollection>,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
struct Product {
    name: Option<String>,
    product_type: Option<String>,
    branding_nft: Option<NFTData>,
    brand: Option<String>,
    price: Option<u64>,
    total_amount: Option<u64>,
    profit_margin: Option<u16>,
    img: FileType,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
struct UserInfo {
    user: String,
    status: Status,
    name: String,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
struct FileType  {
   file:Vec<u8>,
   file_type:String
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
struct NftCollection {
    symbol: String,
    name: String,
    description: String,
    logo: FileType,
    supply_cap: u64,
    creator: String,
    burn_account: String,
    nfts: Vec<NFTData>,
}

#[derive(Clone, CandidType, Serialize, Deserialize, PartialEq)]
enum Status {
    User,
    Creator,
    Brand,
}


#[derive(Default)]
struct State {
    bid: BTreeMap<String, u64>,
    // collections: Vec<NftCollection>,
    nft_array: Vec<NFTData>,
    user_collection: BTreeMap<String, Vec<NftCollection>>,
    users: BTreeMap<String, Status>,
    address_brands: BTreeMap<String, Brand>,
    // user_role: BTreeMap<String, Role>,
    address_creator: BTreeMap<String, Creator>,
    listed_nfts: Vec<NftListing>,
    product_list: Vec<Product>,
    user_balance: BTreeMap<String, u64>,
}

thread_local! {
    static STATE: std::cell::RefCell<State> = std::cell::RefCell::new(State::default());
}


#[init]
fn init() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        *state = State::default();
    });
}

#[update]
fn signup(user: UserInfo) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.users.get(&user.user).is_none() {
            let user_id = user.user.clone(); // Clone the user ID
            match user.status {
                Status::Brand => {
                    state.address_brands.insert(
                        user_id.clone(), // Use the cloned user ID
                        Brand {
                            name: user.name.clone(),
                            rented: Vec::new(),
                            products: Vec::new(),
                        },
                    );
                }
                Status::Creator => {
                    state.address_creator.insert(
                        user_id.clone(), // Use the cloned user ID
                        Creator {
                            name: user.name.clone(),
                            collections: Vec::new(),
                        },
                    );
                }
                _ => {}
            };
            state.users.insert(user_id.clone(), user.status); // Use the cloned user ID
            state.user_balance.insert(user_id, 200000); // Use the cloned user ID
        }
    });
}


#[update]
async fn create_nft_collection(collection_info: NftCollection ,caller:String) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.users.get(&caller) == Some(&Status::Creator) {
            println!("true");
            state.user_collection.entry(caller).or_default().push(collection_info);
        }
    });
}

#[update]
async fn mint_into_collection(index: usize, nft: NFTData,caller:String) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(collections) = state.user_collection.get_mut(&caller) {
            if collections.len() > index && collections[index].nfts.is_empty() {
                collections[index].nfts.push(nft.clone());
                state.nft_array.push(nft)
            }
        }
    });
}

#[query]
async fn get_user_collection(caller:String) -> Vec<NftCollection> {
    STATE.with(|state| state.borrow().user_collection.get(&caller).cloned().unwrap_or_default())
}

#[query]
async fn get_user_role(caller:String) -> Option<Status> {
    STATE.with(|state| state.borrow().users.get(&caller).cloned())
}

#[query]
async fn get_collection_nfts(caller: String, index: usize) -> Option<Vec<NFTData>> {
    STATE.with(|state| {
        let state = state.borrow();
        state.user_collection.get(&caller).and_then(|collections| {
            collections.get(index).map(|collection| collection.nfts.clone())
        })
    })
}


#[query]
async fn get_user_nfts(caller: String) -> Vec<NFTData> {
    STATE.with(|state| {
        let state = state.borrow();
        state.nft_array.iter()
            .filter(|nft| nft.owner == caller)
            .cloned()
            .collect()
    })
}


#[query]
async fn get_listed_nfts() -> Option<Vec<NftListing>> {
    STATE.with(|state| {
        let state = state.borrow();
        let nfts: Vec<NftListing> = state.listed_nfts.iter().cloned().collect();
        if nfts.is_empty() {
            None
        } else {
            Some(nfts)
        }
    })
}


#[update]
async fn list_nft(nft: NFTData, listing_price: u64,caller:String) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let listing = NftListing {
            nft,
            owner: caller,
            price: listing_price,
            bids: state.bid.clone(),
        };
        state.listed_nfts.push(listing);
    });
}

#[update]
async fn buy_nft(index: usize,caller:String) {

    // Step 1: Check balance and listing price with immutable borrow
    let (sufficient_funds, price) = STATE.with(|state| {
        let state = state.borrow();
        let balance = *state.user_balance.get(&caller).unwrap_or(&0);
        let price = state.listed_nfts.get(index).map(|nft| nft.price);
        
        price.map(|p| (balance >= p, p)).unwrap_or((false, 0))
    });

    // Proceed only if funds are sufficient
    if sufficient_funds {
        // Step 2: Deduct balance in a separate mutable borrow
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if let Some(balance) = state.user_balance.get_mut(&caller) {
                *balance -= price;
            }
        });

        // Step 3: Update the owner of the NFT in another separate mutable borrow
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if let Some(nft_listing) = state.listed_nfts.get_mut(index) {
                nft_listing.owner = caller;
            }
        });
    }
}


#[update]
async fn bid_nft(amount: u64, index: usize,caller:String) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(nft_listing) = state.listed_nfts.get_mut(index) {
            nft_listing.bids.insert(caller, amount);
        }
    });
}

#[update]
async fn rent_nft(index: usize,caller:String) {

    // Step 1: Check if NFT listing exists and retrieve necessary values
    let (nft_data, rental_price) = STATE.with(|state| {
        let state = state.borrow();
        state.listed_nfts.get(index).map(|nft_listing| {
            (nft_listing.nft.clone(), nft_listing.price / 10)
        })
    }).unwrap_or_default();

    // Step 2: Check if the user has enough balance
    let has_sufficient_balance = STATE.with(|state| {
        let state = state.borrow();
        state.user_balance.get(&caller).map(|&balance| balance >= rental_price).unwrap_or(false)
    });

    // Only proceed if user has sufficient balance and listing exists
    if has_sufficient_balance {
        // Step 3: Deduct the rental fee from the user balance
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if let Some(balance) = state.user_balance.get_mut(&caller) {
                *balance -= rental_price;
            }
        });

        // Step 4: Add the NFT to the brand's rented list
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if let Some(brand) = state.address_brands.get_mut(&caller) {
                brand.rented.push(nft_data);
            }
        });
    }
}


#[update]
async fn list_product(product: Product,caller:String) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(Status::Brand) = state.users.get(&caller) {
            if let Some(brand) = state.address_brands.get_mut(&caller) {
                brand.products.push(product.clone());
            }
            state.product_list.push(product);
        }
    });
}

#[update]
async fn buy_product(amount: u64, index: usize,caller:String) {

    // Step 1: Retrieve product details and check if the amount is available
    let (product_exists, product_price, _total_amount) = STATE.with(|state| {
        let state = state.borrow();
        if let Some(product) = state.product_list.get(index) {
            if let (Some(total_amount), Some(price)) = (product.total_amount, product.price) {
                return (total_amount >= amount, price, total_amount);
            }
        }
        (false, 0, 0)
    });

    // Step 2: Check if user has sufficient funds
    let has_sufficient_funds = STATE.with(|state| {
        let state = state.borrow();
        state.user_balance.get(&caller).map_or(false, |&balance| balance >= product_price * amount)
    });

    // Proceed only if product exists, total amount is sufficient, and user has funds
    if product_exists && has_sufficient_funds {
        // Step 3: Update the total amount of the product in separate scope
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if let Some(product) = state.product_list.get_mut(index) {
                if let Some(total_amount) = product.total_amount {
                    product.total_amount = Some(total_amount - amount);
                }
            }
        });

        // Step 4: Deduct user's balance and execute revenue sharing
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if let Some(paying) = state.user_balance.get_mut(&caller) {
                *paying -= product_price * amount;
            }
        });

        // Step 5: Execute revenue sharing in its own scope
        STATE.with(|state| {
            let state = state.borrow();
            if let Some(product) = state.product_list.get(index) {
                if let (Some(nft), Some(brand), Some(margin)) = (&product.branding_nft, &product.brand, product.profit_margin) {
                    share_revenue(product_price, amount, nft.creator.clone(), nft.owner.clone(), brand.clone(), margin);
                }
            }
        });
    }
}


fn share_revenue(price: u64, amount: u64, nft_creator: String, nft_owner: String, brand: String, profit_margin: u16) {
    let profit = (price * amount * profit_margin as u64) / 100;
    let royalty = profit / 10;
    let owner_share = (royalty * 35) / 100;
    let creator_share = (royalty * 65) / 100;

    transfer(royalty * 9, brand);
    transfer(owner_share, nft_owner);
    transfer(creator_share, nft_creator);
}

fn transfer(amount: u64, user: String) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let balance = state.user_balance.entry(user).or_insert(0);
        *balance += amount;
    });
}

#[query]
fn __get_candid_interface_tmp_hack() -> String {
    export_service!();  // Exports the full Candid interface for all public functions
    __export_service()
}
