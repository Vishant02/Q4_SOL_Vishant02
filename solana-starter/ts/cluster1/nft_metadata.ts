import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const metadata = {
            name: "Boom-Boom",
            symbol: "BMB",
            description: "Boom Boom unique NFT.",
            image: "https://devnet.irys.xyz/BLUyToF6MWksjq26B5yiqsByZQd9H9QsLx8234FEZ2to",
            attributes: [
                {trait_type: 'Rarity', value: 'Common'},
                {trait_type: 'Type', value: 'Boom Boom'},
                {trait_type: 'Background', value: 'White'},
                {trait_type: 'Color', value: 'Black'},
                {trait_type: 'Size', value: 'Medium'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://devnet.irys.xyz/BLUyToF6MWksjq26B5yiqsByZQd9H9QsLx8234FEZ2to"
                    },
                ]
            },
            creators: [ {
                address: keypair.publicKey,
                share: 100
            }]
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri.replace("arweave.net", "devnet.irys.xyz"));
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
