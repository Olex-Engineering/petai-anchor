
# PETAI program  
This is solana programa for PETAI game

## TODO
First of all need to create admin panel of the game
  1. Create token for game
  2. Create NFT collections/update
  3. Minting NFT's to collections (transfer authority to program)/update
  4. Create real dogs array
  4. Init program states/update
  5. Printing NFT's through program
  6. In month report of charity to your concret dog

After we create a programs with instructions
1. Init program state
  * 1.1 Add dogs NFT's array { key, price }
  * * 1.1.2 Add real dogs array { wallet, url }
  * 1.2 Add bg's NFT's array { key, price }
  * 1.3 Add foods NFT's { key, price }
  * 1.4 Add toys NFT's { key, price }
  * 1.5 Add interior NFT's { key, price }
  * 1.6 Add token mint address
  * 1.7 Set authority
  * (FUTURE!) add exterior for walk
   
2. Add possibility to random print edition for (dogs, bg's, interior) collection
  * 2.1 Link print dog edition with random real dog
3. Add possibility to buy all NFT's edition
  * 1.1 Burn 80% procent of token, other to charity (example)
   
4. Update program state
  * 1.1 Check authority (only game creator wallet)
5. Init interior state (dog can destroy interior NFT)
  * Poops amount
  * Items in vector = {key, state}
6. Init user state
  * 1.1 Add validator to check if user has NFT in pet collection and bg collections
  * 1.2 Add user current bg
  * 1.3 Add user pet
7. Update user state
8. Init pet state
  * 1.1 Set empty love, loneliness, happy, hungry stats
  * 1.2 Add empty effects
  * 1.3 Set global state (HAPPY, DIED, ETC...)
  * 1.4 Run clockwork
9. Add play instruction (increase love, happy)
10. Add food instruction (increase food)
11. Add show love instruction (increase love)
12. Add clockwork and update pet state in every (For example: 2 hour)
  * Check if user has some "BENEFIT" NFT's (if yes > smaller stats decreasing)
  * Decrease pets stats
  * Change interior state
  * If dog is happy -> mint some amount of tokens to user (70% of what user need to continue have dog in happy mood)

// FUTURE
count human steps and convert it into dog happiness

// FUTURE
add wallet session keys

// FUTURE
add AI

// FUTURE
Add events with other user

// FUTURE sync with web2
Mint token with current price through "In app purchare"
Add variant to add AD's to the game (send money from the ad's to players)
