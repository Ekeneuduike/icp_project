import { useState } from 'react';
import { icp_project_backend } from 'declarations/icp_project_backend';

function App() {
  const [greeting, setGreeting] = useState('');

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    icp_project_backend.greet(name).then((greeting) => {
      setGreeting(greeting);
    });
    return false;
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>
      <section id="greeting">{greeting}</section>
    </main>
  );
}

export default App;



// import { useState } from 'react';
// import { icp_project_backend } from '../.././declarations/icp_project_backend';

// enum UserRole {
//   User = 'User',
//   Creator = 'Creator',
//   Brand = 'Brand',
// }

// type NFTData = {
//   id: number;
//   collections_id: number;
//   creator: string;
//   owner: string;
//   metadata: string;
//   name: string;
// };

// type NftCollection = {
//   creator: string;
//   supply_cap: number;
//   burn_account: string;
//   logo: string;
//   name: string;
//   nfts: NFTData[];
//   description: string;
//   symbol: string;
// };

// type Product = {
//   total_amount?: number;
//   name?: string;
//   product_type?: string;
//   brand?: string;
//   price?: number;
//   profit_margin?: number;
//   branding_nft?: NFTData;
// };

// function App() {
//   const [role, setRole] = useState<UserRole | null>(null);
//   const [name, setName] = useState('');
//   const [collections, setCollections] = useState<NftCollection[]>([]);
//   const [products, setProducts] = useState<Product[]>([]);

//   const handleSignIn = async () => {
//     if (!role) return;
//     await icp_project_backend.signup({ name, user: 'some-principal', status: role });
//     // Set initial data
//     if (role === UserRole.Creator) {
//       const userCollections = await icp_project_backend.get_user_collection();
//       setCollections(userCollections);
//     }
//   };

//   // Role-based Dashboard Components
//   const UserDashboard = () => (
//     <div>
//       <h3>Welcome, {name}! This is your User Dashboard.</h3>
//       <p>Here you can view your NFT collections and statistics.</p>
//       {/* User's Collection Display */}
//       <div>
//         {collections.map((collection) => (
//           <div key={collection.name}>
//             <h4>{collection.name}</h4>
//             <p>{collection.description}</p>
//           </div>
//         ))}
//       </div>
//     </div>
//   );

//   const CreatorDashboard = () => {
//     const createCollection = async () => {
//       const newCollection: NftCollection = {
//         creator: 'some-principal',
//         supply_cap: 1000,
//         burn_account: 'some-burn-account',
//         logo: 'logo.png',
//         name: 'New NFT Collection',
//         nfts: [],
//         description: 'This is a new collection',
//         symbol: 'NEW',
//       };
//       await icp_project_backend.create_nft_collection(newCollection);
//       const updatedCollections = await icp_project_backend.get_user_collection();
//       setCollections(updatedCollections);
//     };

//     const mintNFT = async (collectionId: number) => {
//       const newNFT: NFTData = {
//         id: Date.now(),
//         collections_id: collectionId,
//         creator: 'some-principal',
//         owner: 'some-principal',
//         metadata: 'NFT metadata',
//         name: 'New NFT',
//       };
//       await icp_project_backend.mint_into_collection(collectionId, newNFT);
//       const updatedCollections = await icp_project_backend.get_user_collection();
//       setCollections(updatedCollections);
//     };

//     return (
//       <div>
//         <h3>Welcome, {name}! This is your Creator Dashboard.</h3>
//         <button onClick={createCollection}>Create New Collection</button>
//         <div>
//           {collections.map((collection) => (
//             <div key={collection.name}>
//               <h4>{collection.name}</h4>
//               <p>{collection.description}</p>
//               <button onClick={() => mintNFT(collection.creator)}>Mint NFT into Collection</button>
//             </div>
//           ))}
//         </div>
//       </div>
//     );
//   };

//   const BrandDashboard = () => {
//     const listProductForSale = async () => {
//       const newProduct: Product = {
//         total_amount: 100,
//         name: 'New Branded Product',
//         product_type: 'Merchandise',
//         brand: 'some-principal',
//         price: 500,
//         profit_margin: 10,
//         branding_nft: collections[0]?.nfts[0],
//       };
//       await icp_project_backend.list_product(newProduct);
//       const updatedProducts = await icp_project_backend.get_user_collection();
//       setProducts(updatedProducts);
//     };

//     return (
//       <div>
//         <h3>Welcome, {name}! This is your Brand Dashboard.</h3>
//         <button onClick={listProductForSale}>List New Branded Product</button>
//         <div>
//           {products.map((product, index) => (
//             <div key={index}>
//               <h4>{product.name}</h4>
//               <p>Price: {product.price}</p>
//               <p>Profit Margin: {product.profit_margin}%</p>
//             </div>
//           ))}
//         </div>
//       </div>
//     );
//   };

//   if (!role) {
//     return (
//       <div>
//         <h2>Sign In</h2>
//         <input
//           type="text"
//           placeholder="Enter your name"
//           value={name}
//           onChange={(e) => setName(e.target.value)}
//         />
//         <select value={role || ''} onChange={(e) => setRole(e.target.value as UserRole)}>
//           <option value="">Select Role</option>
//           <option value={UserRole.User}>User</option>
//           <option value={UserRole.Creator}>Creator</option>
//           <option value={UserRole.Brand}>Brand</option>
//         </select>
//         <button onClick={handleSignIn}>Sign In</button>
//       </div>
//     );
//   }

//   return (
//     <div>
//       {role === UserRole.User && <UserDashboard />}
//       {role === UserRole.Creator && <CreatorDashboard />}
//       {role === UserRole.Brand && <BrandDashboard />}
//     </div>
//   );
// }

// export default App;

