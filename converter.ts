import * as fs from "fs";
import bs58 from "bs58";

// Replace with your Base58 secret key
const base58SecretKey =
  "2pSgfJc8Nu2UJ9HjBtQFLnQi6wN7bU7NqzGbLYw3H7rJi85fE6NhWeBois9oBiYJvUQQY6v1cGVGGdMQ52VdD2Jc";

try {
  // Decode Base58 string to byte array
  const secretKeyBytes = bs58.decode(base58SecretKey);

  // Convert Uint8Array to number array
  const secretKeyArray = Array.from(secretKeyBytes);

  // Write to JSON file
  fs.writeFileSync("secret.json", JSON.stringify(secretKeyArray));

  console.log("✅ Secret key saved to secret.json");
} catch (error) {
  console.error("❌ Error decoding Base58:", error);
}
