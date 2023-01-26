"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[77170],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>u});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var s=a.createContext({}),c=function(e){var t=a.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=c(e.components);return a.createElement(s.Provider,{value:t},e.children)},m="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},h=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),m=c(n),h=r,u=m["".concat(s,".").concat(h)]||m[h]||d[h]||o;return n?a.createElement(u,i(i({ref:t},p),{},{components:n})):a.createElement(u,i({ref:t},p))}));function u(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=h;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[m]="string"==typeof e?e:r,i[1]=l;for(var c=2;c<o;c++)i[c]=n[c];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}h.displayName="MDXCreateElement"},92452:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>i,default:()=>m,frontMatter:()=>o,metadata:()=>l,toc:()=>c});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:4,title:"Shares contract"},i=void 0,l={unversionedId:"smart-contracts/example/shares",id:"version-1.7.0/smart-contracts/example/shares",title:"Shares contract",description:"Similarly, we will implement another PSP-22 token",source:"@site/versioned_docs/version-1.7.0/smart-contracts/example/shares.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/shares",permalink:"/1.7.0/smart-contracts/example/shares",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-1.7.0/smart-contracts/example/shares.md",tags:[],version:"1.7.0",sidebarPosition:4,frontMatter:{sidebar_position:4,title:"Shares contract"},sidebar:"tutorialSidebar",previous:{title:"Implement PSP-22 contract",permalink:"/1.7.0/smart-contracts/example/psp22"},next:{title:"Loan contract",permalink:"/1.7.0/smart-contracts/example/loan"}},s={},c=[{value:"Definition of the <code>Shares</code> trait",id:"definition-of-the-shares-trait",level:2},{value:"Add dependencies",id:"add-dependencies",level:2},{value:"Implement the contract",id:"implement-the-contract",level:2},{value:"Define the storage",id:"define-the-storage",level:2},{value:"Implement the extension traits",id:"implement-the-extension-traits",level:2},{value:"Implement the Burnable and Mintable traits",id:"implement-the-burnable-and-mintable-traits",level:2},{value:"Define the constructor",id:"define-the-constructor",level:2}],p={toc:c};function m(e){let{components:t,...n}=e;return(0,r.kt)("wrapper",(0,a.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"Similarly, we will implement another ",(0,r.kt)("a",{parentName:"p",href:"/1.7.0/smart-contracts/PSP22/"},"PSP-22")," token\nwhich will represent the ownership of assets available by the smart contract\nto be lent. In this token, we will need ",(0,r.kt)("a",{parentName:"p",href:"/1.7.0/smart-contracts/PSP22/Extensions/metadata"},"PSP-22 Metadata"),"\nand we will also need to mint and burn this token. We only want our contract(lending contract) to\nperform these actions, so we will also add the ",(0,r.kt)("a",{parentName:"p",href:"/1.7.0/smart-contracts/ownable"},"Ownable")," extension."),(0,r.kt)("h2",{id:"definition-of-the-shares-trait"},"Definition of the ",(0,r.kt)("inlineCode",{parentName:"h2"},"Shares")," trait"),(0,r.kt)("p",null,"In the ",(0,r.kt)("inlineCode",{parentName:"p"},"traits/shares.rs"),", we will define a ",(0,r.kt)("inlineCode",{parentName:"p"},"Shares")," trait.\nThat trait contains the next super traits: ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Mintable"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Burnable"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Metadata"),", and ",(0,r.kt)("inlineCode",{parentName:"p"},"Ownable"),", without any other method.\nThat shows that ",(0,r.kt)("inlineCode",{parentName:"p"},"Shares")," is ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," with mint and burn methods that can be called only by the owner.\nIn the implementation of the contract, we will implement that trait to be sure that all super traits are also implemented.\n",(0,r.kt)("inlineCode",{parentName:"p"},"SharesRef")," can be used by other developers to do a cross contract call to ",(0,r.kt)("inlineCode",{parentName:"p"},"SharesContract"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"use brush::contracts::traits::{\n    ownable::*,\n    psp22::{\n        extensions::{\n            burnable::*,\n            metadata::*,\n            mintable::*,\n        },\n        *,\n    },\n};\n\n#[brush::wrapper]\npub type SharesRef = dyn PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable;\n\n#[brush::trait_definition]\npub trait Shares: PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable {}\n")),(0,r.kt)("h2",{id:"add-dependencies"},"Add dependencies"),(0,r.kt)("p",null,"In addition to the dependencies imported in the ",(0,r.kt)("a",{parentName:"p",href:"/1.7.0/smart-contracts/PSP22/"},"PSP-22"),"\ndocumentation, we will also add the ",(0,r.kt)("inlineCode",{parentName:"p"},"ownable")," dependency the same way as in the\n",(0,r.kt)("a",{parentName:"p",href:"/1.7.0/smart-contracts/ownable"},"ownable")," documentation. We will be using ",(0,r.kt)("inlineCode",{parentName:"p"},"SharesContract"),"\nas a dependency in our lending contract to instantiate it. So we need to also add\nthe ",(0,r.kt)("inlineCode",{parentName:"p"},'"rlib"')," crate type to have the ability to import the ",(0,r.kt)("inlineCode",{parentName:"p"},"SharesContract")," as a dependency."),(0,r.kt)("h2",{id:"implement-the-contract"},"Implement the contract"),(0,r.kt)("p",null,"Implementing our shares contract will follow the same steps as implementing\nthe basic ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP-22")," contract in the previous step, but we will do some small\nchanges for the token to be mintable, burnable, and for these functions to\nbe restricted. Therefore, on top of the imports in the previous contract,\nwe also need these imports:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n/// This contract will be used to represent the shares of a user\n/// and other instance of this contract will be used to represent\n/// the amount of borrowed tokens\n#[brush::contract]\npub mod shares {\n    use brush::contracts::{\n        ownable::*,\n        psp22::extensions::{\n            burnable::*,\n            metadata::*,\n            mintable::*,\n        },\n    };\n\n    use brush::modifiers;\n\n    use ink_lang::codegen::Env;\n\n    use ink_prelude::string::String;\n    use ink_storage::traits::SpreadAllocate;\n\n    use lending_project::traits::shares::*;\n')),(0,r.kt)("h2",{id:"define-the-storage"},"Define the storage"),(0,r.kt)("p",null,"In this storage, we will also derive the storage trait related to ",(0,r.kt)("inlineCode",{parentName:"p"},"Ownable"),"\nand declare the field related to this trait."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"/// Define the storage for PSP22 data, Metadata data and Ownable data\n#[ink(storage)]\n#[derive(Default, SpreadAllocate, PSP22Storage, OwnableStorage, PSP22MetadataStorage)]\npub struct SharesContract {\n    #[PSP22StorageField]\n    psp22: PSP22Data,\n    #[OwnableStorageField]\n    ownable: OwnableData,\n    #[PSP22MetadataStorageField]\n    metadata: PSP22MetadataData,\n}\n")),(0,r.kt)("h2",{id:"implement-the-extension-traits"},"Implement the extension traits"),(0,r.kt)("p",null,"We will be using these extensions in our token, so we will implement them for\nour storage."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"// implement PSP22 Trait for our share\nimpl PSP22 for SharesContract {}\n\n// implement Ownable Trait for our share\nimpl Ownable for SharesContract {}\n\n// implement Metadata Trait for our share\nimpl PSP22Metadata for SharesContract {}\n\n// It forces the compiler to check that you implemented all super traits\nimpl Shares for SharesContract {}\n")),(0,r.kt)("h2",{id:"implement-the-burnable-and-mintable-traits"},"Implement the Burnable and Mintable traits"),(0,r.kt)("p",null,"Now we will implement the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Burnable")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Mintable")," traits.\nThese are a little different so we are doing it in a separate section.\nWe don't want anybody to mint or burn the tokens, we only want the owner,\nin this case, our lending contract, to do it. So we will add the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Burnable"),"\nand ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Mintable")," and mark the functions of these traits with the ",(0,r.kt)("inlineCode",{parentName:"p"},"only_owner"),"\nrestriction."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"// implement Mintable Trait for our share\nimpl PSP22Mintable for SharesContract {\n    /// override the `mint` function to add the `only_owner` modifier\n    #[ink(message)]\n    #[modifiers(only_owner)]\n    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {\n        self._mint(account, amount)\n    }\n}\n\n// implement Burnable Trait for our share\nimpl PSP22Burnable for SharesContract {\n    /// override the `burn` function to add the `only_owner` modifier\n    #[ink(message)]\n    #[modifiers(only_owner)]\n    fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {\n        self._burn(self.env().caller(), amount)\n    }\n\n    /// override the `burn_from` function to add the `only_owner` modifier\n    #[ink(message)]\n    #[modifiers(only_owner)]\n    fn burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {\n        self._burn_from(account, amount)\n    }\n}\n")),(0,r.kt)("p",null,"This will restrict accounts other than the owner of the token (which will be the lending contract)\nfrom calling these functions."),(0,r.kt)("h2",{id:"define-the-constructor"},"Define the constructor"),(0,r.kt)("p",null,"Finally, we will define the constructor where we will set the name and the symbol\nof the token and then initialize the owner of the token\n(which then will be able to mint and burn the tokens)."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl SharesContract {\n    /// constructor with name and symbol\n    #[ink(constructor)]\n    pub fn new(name: Option<String>, symbol: Option<String>) -> Self {\n        ink_lang::codegen::initialize_contract(|instance: &mut SharesContract| {\n            let caller = instance.env().caller();\n            instance.metadata.name = name;\n            instance.metadata.symbol = symbol;\n            instance.metadata.decimals = 18;\n            instance._init_with_owner(caller);\n        })\n    }\n}\n")))}m.isMDXComponent=!0}}]);