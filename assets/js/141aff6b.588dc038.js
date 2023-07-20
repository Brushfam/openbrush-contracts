"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[50465],{3905:(t,e,n)=>{n.d(e,{Zo:()=>p,kt:()=>f});var a=n(67294);function r(t,e,n){return e in t?Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}):t[e]=n,t}function o(t,e){var n=Object.keys(t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(t);e&&(a=a.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),n.push.apply(n,a)}return n}function i(t){for(var e=1;e<arguments.length;e++){var n=null!=arguments[e]?arguments[e]:{};e%2?o(Object(n),!0).forEach((function(e){r(t,e,n[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(n,e))}))}return t}function l(t,e){if(null==t)return{};var n,a,r=function(t,e){if(null==t)return{};var n,a,r={},o=Object.keys(t);for(a=0;a<o.length;a++)n=o[a],e.indexOf(n)>=0||(r[n]=t[n]);return r}(t,e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(t);for(a=0;a<o.length;a++)n=o[a],e.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(t,n)&&(r[n]=t[n])}return r}var s=a.createContext({}),u=function(t){var e=a.useContext(s),n=e;return t&&(n="function"==typeof t?t(e):i(i({},e),t)),n},p=function(t){var e=u(t.components);return a.createElement(s.Provider,{value:e},t.children)},c="mdxType",m={inlineCode:"code",wrapper:function(t){var e=t.children;return a.createElement(a.Fragment,{},e)}},d=a.forwardRef((function(t,e){var n=t.components,r=t.mdxType,o=t.originalType,s=t.parentName,p=l(t,["components","mdxType","originalType","parentName"]),c=u(n),d=r,f=c["".concat(s,".").concat(d)]||c[d]||m[d]||o;return n?a.createElement(f,i(i({ref:e},p),{},{components:n})):a.createElement(f,i({ref:e},p))}));function f(t,e){var n=arguments,r=e&&e.mdxType;if("string"==typeof t||r){var o=n.length,i=new Array(o);i[0]=d;var l={};for(var s in e)hasOwnProperty.call(e,s)&&(l[s]=e[s]);l.originalType=t,l[c]="string"==typeof t?t:r,i[1]=l;for(var u=2;u<o;u++)i[u]=n[u];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},85725:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>s,contentTitle:()=>i,default:()=>m,frontMatter:()=>o,metadata:()=>l,toc:()=>u});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:6,title:"Data and derive macro"},i=void 0,l={unversionedId:"smart-contracts/example/data",id:"version-3.1.1/smart-contracts/example/data",title:"Data and derive macro",description:"Data segregation",source:"@site/versioned_docs/version-3.1.1/smart-contracts/example/data.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/data",permalink:"/openbrush-contracts/3.1.1/smart-contracts/example/data",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-3.1.1/smart-contracts/example/data.md",tags:[],version:"3.1.1",sidebarPosition:6,frontMatter:{sidebar_position:6,title:"Data and derive macro"},sidebar:"tutorialSidebar",previous:{title:"Loan contract",permalink:"/openbrush-contracts/3.1.1/smart-contracts/example/loan"},next:{title:"Lending impls",permalink:"/openbrush-contracts/3.1.1/smart-contracts/example/impls"}},s={},u=[{value:"Data segregation",id:"data-segregation",level:2},{value:"Storage trait",id:"storage-trait",level:3},{value:"Data of the trait",id:"data-of-the-trait",level:3},{value:"Default implementation",id:"default-implementation",level:3},{value:"&quot;Inheritance&quot; of the implementation",id:"inheritance-of-the-implementation",level:3}],p={toc:u},c="wrapper";function m(t){let{components:e,...n}=t;return(0,r.kt)(c,(0,a.Z)({},p,n,{components:e,mdxType:"MDXLayout"}),(0,r.kt)("h2",{id:"data-segregation"},"Data segregation"),(0,r.kt)("p",null,'Rust doesn\'t have inheritance like OOP languages.\nIf you want to "inherit" some fields, you can use structural composition.\nIf you want to "inherit" some implementation, you can use traits.\nTraits can have a ',(0,r.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations"},"default implementation")," or a ",(0,r.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods"},"generic implementation"),".\nThe traits in Rust can't contain fields, it is pure interfaces."),(0,r.kt)("p",null,"Based on that information we propose you the following concept of smart contract\ndevelopment:"),(0,r.kt)("h3",{id:"storage-trait"},"Storage trait"),(0,r.kt)("p",null,"Extract the logic of data storing into a separate trait to have the ability to\ndefine the default implementation without knowing what contract will inherit that.\nYou can use that separate trait as a bound in your generic implementation(below we will describe)."),(0,r.kt)("p",null,"You can define your own storage trait like:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub trait PointStorage {\n    fn get(&self) -> & PointData;\n    fn get_mut(&mut self) -> &mut PointData;\n}\n")),(0,r.kt)("p",null,"Or you can use ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::Storage")," trait from OpenBrush."),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"Storage")," is a generic trait, so you can use it to work with different storage.\nFor example, if in your default implementation you need to have ",(0,r.kt)("inlineCode",{parentName:"p"},"psp22::extensions::metadata::Data")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"psp22::Data"),",\nyou can add bounds ",(0,r.kt)("inlineCode",{parentName:"p"},"T: Storage<metadata::Data> + Storage<psp22::Data>"),".\nIt allows you to work with two independent storage."),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::Storage")," trait requires that the inner data implements the\n",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::OccupyStorage")," trait.\nIt is because each storage in the smart contract should occupy a unique storage key.\nOverlapping of those keys can cause unexpected bugs. Derive macro provided by\nOpenBrush to simplify the implementation of the ",(0,r.kt)("inlineCode",{parentName:"p"},"Storage")," trait also checks that\nthe storage key from the ",(0,r.kt)("inlineCode",{parentName:"p"},"OccupyStorage ")," trait is unique."),(0,r.kt)("h3",{id:"data-of-the-trait"},"Data of the trait"),(0,r.kt)("p",null,"That trait returns some data with fields that can be used in the implementation.\nThe data is a simple struct with fields. Later that struct can be embedded into the contract struct."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub struct PointData {\n    pub x: u32,\n    pub y: u32,\n}\n")),(0,r.kt)("p",null,"If you want to use ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::Storage")," then you also need to implement ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::OccupyStorage"),"\nwith unique storage key."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub struct PointData {\n    pub x: u32,\n    pub y: u32,\n}\n\nimpl openbrush::traits::OccupyStorage for PointData {\n    // You can specify your unique key manually like `0x123` or you can use macro\n    const KEY: u32 = openbrush::storage_unique_key!(PointData);\n}\n")),(0,r.kt)("p",null,"Also, you can use the ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::upgradeable_storage")," macro that implements that trait by default,\nand also prepare the storage to be upgradeable."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(PointData))]\npub struct PointData {\n    pub x: u32,\n    pub y: u32,\n}\n")),(0,r.kt)("h3",{id:"default-implementation"},"Default implementation"),(0,r.kt)("p",null,"Define the default or generic implementation for your main trait with the restriction that ",(0,r.kt)("inlineCode",{parentName:"p"},"Self"),"\nshould also implement storage trait."),(0,r.kt)("p",null,"A default implementation:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'pub trait Point: PointStorage {\n    fn x(&self) -> u32 {\n        PointStorage::get(self).x\n    }\n    \n    fn y(&self) -> u32 {\n        PointStorage::get(self).y\n    }\n    \n    fn name(&self) -> String {\n        "AlphaPoint".to_string()\n    }\n}\n')),(0,r.kt)("p",null,"or a generic implementation:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![feature(min_specialization)]\n\npub trait Point {\n    fn x(&self) -> u32;\n\n    fn y(&self) -> u32;\n\n    fn name(&self) -> String;\n}\n\nimpl<T: PointStorage> Point for T {\n    default fn x(&self) -> u32 {\n        PointStorage::get(self).x\n    }\n\n    default fn y(&self) -> u32 {\n        PointStorage::get(self).y\n    }\n\n    default fn name(&self) -> String {\n        "AlphaPoint".to_string()\n    }\n}\n')),(0,r.kt)("p",null,"A default implementation with ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::Storage"),":"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'pub trait Point: openbrush::traits::Storage<PointData> {\n    fn x(&self) -> u32 {\n        self.data().x\n    }\n    \n    fn y(&self) -> u32 {\n        self.data().y\n    }\n    \n    fn name(&self) -> String {\n        "AlphaPoint".to_string()\n    }\n}\n')),(0,r.kt)("p",null,"or a generic implementation with ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::Storage"),":"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![feature(min_specialization)]\n\npub trait Point {\n    fn x(&self) -> u32;\n\n    fn y(&self) -> u32;\n\n    fn name(&self) -> String;\n}\n\nimpl<T: openbrush::traits::Storage<PointData>> Point for T {\n    default fn x(&self) -> u32 {\n        self.data().x\n    }\n\n    default fn y(&self) -> u32 {\n        self.data().y\n    }\n\n    default fn name(&self) -> String {\n        "AlphaPoint".to_string()\n    }\n}\n')),(0,r.kt)("h3",{id:"inheritance-of-the-implementation"},'"Inheritance" of the implementation'),(0,r.kt)("p",null,'When someone wants to "inherit" implementation and fields, he can embed the data structure,\nimplement the storage trait, and define an impl section of the main trait:'),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"struct PointContract {\n    point: PointData,\n}\n\nimpl PointStorage for PointContract {\n    fn get(&self) -> & PointData {\n        &self.point\n    }\n    fn get_mut(&mut self) -> &mut PointData {\n        &mut self.point\n    }\n}\n\nimpl Point for PointContract {}\n")),(0,r.kt)("p",null,"If you are using ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::Storage")," and your data implements ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::traits::OccupyStorage"),"\ntrait, then you can use derive macro to automate the implementation of the trait.\nEach field for which you want to implement the ",(0,r.kt)("inlineCode",{parentName:"p"},"Storage")," trait should be marked with ",(0,r.kt)("inlineCode",{parentName:"p"},"#[storage_field]"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"use openbrush::traits::Storage;\n\n#[derive(Storage)]\nstruct PointContract {\n    #[storage_field]\n    point: PointData,\n}\n\nimpl Point for PointContract {}\n")))}m.isMDXComponent=!0}}]);