/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

 const isProd = process.env.NODE_ENV === "production";

 // @ts-check
 
 /** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
 const sidebars = {
   // By default, Docusaurus generates a sidebar from the docs folder structure
   // tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],
 
   // But you can create a sidebar manually
   internalSidebar: [{ type: "autogenerated", dirName: "internal" }],
   user_guides: [
     {
       type: "category",
       label: "User Guides",
       link: {
         type: "generated-index",
         slug: "user-guides",
       },
       collapsible: false,
       items: [
         {
           type: "category",
           label: "Accounts and Wallets",
           link: {
             type: "generated-index",
             slug: "accounts-wallets",
           },
           collapsible: false,
           items: [
             "user-guides/polkadotjs-extension-create-account",
             "user-guides/talisman-create-account",
             "user-guides/keplr-guide",
             "user-guides/layr-guide",
           ],
         },
         {
          type: "category",
          label: "Solana Restaking",
          link: {
            type: "generated-index",
            slug: "solana-restaking",
          },
          collapsible: false,
          items: [
            "user-guides/restaking-sol",
            "user-guides/team-competition",
          ],
        },
         {
           type: "category",
           label: "Transactions and Trading",
           link: {
             type: "generated-index",
             slug: "transactions-and-trading",
           },
           collapsible: false,
           items: [
             "user-guides/composable-cosmos-staking",
             "user-guides/how-to-provide-liquidity",
             "user-guides/how-to-trade-pica-on-pablo",
             "user-guides/dot-lp-guide",
             "user-guides/trustless-transfers",
           ],
         },
         {
           type: "category",
           label: "FAQs",
           link: {
             type: "generated-index",
             slug: "faqs",
           },
           collapsible: false,
           items: [
             "user-guides/trustless-faq",
           ],
         }, 
         "user-guides/picasso-governance",
       ],
     },
   ],
   networks: [
     {
       type: "category",
       label: "Picasso",
       link: {
         type: "doc",
         id: "networks/picasso-parachain-overview",
       },
       collapsible: false,
       collapsed: false,
       items: [
         "networks/picasso/governance",
         "networks/picasso/asset-list",
         "networks/picasso/pica-use-cases",
         "networks/picasso/tokenomics",
         "networks/picasso/token-transparency",
         "networks/picasso/crowdloan",
         {
           type: "category",
           label: "CosmWasm",
           link: {
             type: "doc",
             id: "technology/cosmwasm-vm-overview",
           },
           collapsible: true,
           collapsed: true,
           items: [
             "technology/cosmwasm/existing-cosmwasm-projects-to-deploy-on-ccw-vm",
             "technology/cosmwasm/synergy-with-ibc-for-cvm",
             "technology/cosmwasm/writing-smart-contracts-with-cosmwasm",
           ],
         },
         {
           type: "category",
           label: "Apollo",
           link: {
             type: "doc",
             id: "technology/apollo-overview",
           },
           collapsible: true,
           collapsed: true,
           items: [
             "technology/apollo/apollo-how-it-works",
             "technology/apollo/technical-details",
             "technology/apollo/apollo-deployment",
           ],
         },
         {
          type: "category",
          label: "Pablo",
          link: {
            type: "doc",
            id: "technology/pablo-overview",
          },
          collapsible: true,
          collapsed: true,
          items: [
            "technology/pablo/swaps-trading",
            "technology/pablo/launch-pools",
            "technology/pablo/cross-chain-DEX",
          ],
        },
       ],
     },
 
     {
       type: "category",
       label: "Composable",
       link: {
         type: "doc",
         id: "networks/composable-parachain-overview",
       },
       collapsible: false,
       collapsed: false,
       items: [
         "networks/composable/composie-asset-list",
         "networks/composable/composable-crowdloan",
         "networks/composable/LAYR-tokenomics",
         "networks/composable/composable-council",
         {
          type: "category",
          label: "Polkadot Liquid Staking",
          link: {
            type: "doc",
            id: "technology/liquid-staking",
          },
          collapsible: true,
          collapsed: true,
          items: [
            "technology/liquid-staking/why-lsd",
            "technology/liquid-staking/technical-overview",
          ],
        },
       ],
     },
 
     "networks/composable-cosmos",
   ],

   architecture: [
    "technology/architecture",
    {
    type: "category",
       label: "IBC",
       link: {
         type: "doc",
         id: "technology/ibc",
       },
       collapsible: false,
       collapsed: false,
       items: [
        "technology/ibc/ethereum",
        "technology/ibc/solana",
        "technology/ibc/polkadot",
        "technology/ibc/polkadot-kusama",
        "technology/ibc/near",
        "technology/ibc/hyperspace-relayer",
        "technology/ibc/light-clients",
        "technology/ibc/merkle-mountain-ranges",
        "technology/ibc/beefy-light-client",
       ],
      },

    {
      type: "category",
          label: "CVM",
          link: {
           type: "doc",
            id: "technology/cvm",
         },
        collapsible: false,
        collapsed: false,
        items: [
        "technology/cvm/specification",
        "technology/cvm/virtual-wallet",
        "technology/cvm/tutorial",
   ],
  },
  {
    type: "category",
        label: "Solana Restaking",
        link: {
         type: "doc",
          id: "technology/solana-restaking",
       },
      collapsible: false,
      collapsed: false,
      items: [
    "technology/solana-restaking/technical-overview", 
     "technology/solana-restaking/vaults",
     "technology/solana-restaking/mantis-games"
     ],
},

],

mantis: [
  {
    type: "doc",
    id: "technology/mantis",
  },
   "technology/mantis/benefits-use-cases",
   "technology/mantis/protocol-architecture",
   "technology/mantis/protocol-flow",
   "technology/mantis/solvers-solutions",
   "technology/mantis/solver-integration",
   "technology/mantis/solver-guide",
   "technology/mantis/integration-guide",
   "technology/mantis/tools",

],
   develop: [
    {
      type: "doc",
      id: "develop/build-on-composable",
    },
    "develop/composable-cosmos",
    "develop/solana-avs-testnet",
    "develop/local-picasso-guide",
    "develop/oracle-set-up-guide",
    "develop/collator-guide",
    {
      type: "category",
      label: "Cosmwasm CLI",
      link: {
        type: "doc",
        id: "develop/cosmwasm-cli",
      },
      collapsible: true,
      collapsed: true,
      items: ["develop/cosmwasm/walkthrough"],
    },
    {
      type: "category",
      label: "Nix",
      link: {
        type: "doc",
        id: "nix"
      },
      collapsible: true,
      collapsed: true,
      items: [
        "nix/install",
        "nix/run-packages",
        "nix/development-environments",
        "nix/running-checks",
        "nix/reading-logs",
        "nix/defining-your-own-packages",
        "nix/editing-docs",
        "nix/troubleshooting",
      ],
    },
    {
      type: "doc",
      id: "codespaces",
    },

  ],
   ecosystem: [
    {
      type: "doc",
      id: "ecosystem/composable-ecosystem",
    },
     "ecosystem/request-for-proposals",
     "ecosystem/composable-research",
     "ecosystem/the-composable-team",
     "ecosystem/press-kit",
     {
       type: "doc",
       label: "Audits, Fixes & Bug Bounties",
       id: "audits/audit-results-recommendations-and-remediations",
     },
     {
      "type": "category",
      "label": "Archived",
      "collapsible": true,
      "collapsed": true,
      "items": [
        {
          "type": "doc",
          "label": "Mosaic (Discontinued)",
          "id": "technology/mosaic/mosaic-withdrawal-guide"
        },
        {
          "type": "category",
          "label": "Parachain Vault Strategy (Discontinued)",
          "link": {
            "type": "doc",
            "id": "technology/parachain-vault-strategy/composable-strategies-withdrawal-guide"
          },
          "collapsible": true,
          "collapsed": true,
          "items": [
            "technology/parachain-vault-strategy/vault-process-in-detail",
            "technology/parachain-vault-strategy/contracts-technical-details"
          ],
        },
      ],
    },    
   ],
 };
 
 // if (!isProd) {
 //     sidebars.tutorialSidebar.unshift({
 //         type: 'category',
 //         label: 'test-SCDI',
 //         link: {
 //             type: 'doc',
 //             id: 'testSCDI/entry',
 //         },
 //         collapsible: true,
 //         collapsed: true,
 //         items: [
 //             {
 //                 type: 'link',
 //                 label: 'test-SCDI',
 //                 href: '/test-vm',
 //             },
 //         ],
 //     });
 // }
 
 module.exports = sidebars;