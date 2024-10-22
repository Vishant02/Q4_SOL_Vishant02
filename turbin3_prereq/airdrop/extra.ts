import bs58 from "bs58";

// Your private key
const privateKeyBase58 = '2qAUcWgrMtCF4KxY4HchULD2F5FK4EHqSNQWYAL57MqxLBMi7gVvYMAjpmxCPbWGVbQu8eXe5CnDkrZmGRLtWah9';

// Convert the base58 string to a byte array
const privateKeyArray = bs58.decode(privateKeyBase58);

console.log('Private Key Array:', Array.from(privateKeyArray));