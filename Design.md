MMO Idle

- Farm gold in MMOs to sell for money
- Hire people to farm gold

- Player
    - Reborn points
    - Idle tokens
    - Last time saved
        - Used to calculate away time?
        - Increments only
    - Cash
    - Workers
    - Difficulty


- Date
    - There are dates so fees can be paid
    - Bans checks happen daily

- Idle
    - Rewards tokens that are used to increase time speed
    - Speed adjustable
        - +- 2
        - Shortcuts
            - 2x 4x 8x 16x 32x

    
- Workers
    - Traits
        - Type

    - Assignments
        - Farmer
            - Can farm gold

        - Researcher
            - Cheats
                - 
            - Games
                - Unlock game
                - Improve methods to gain gold
                - Reduce base difficulty
                - Reduce base difficulty growth multiplier
                - Improve marketing so more gold is sold per population
        
        - Training
            - Increases player difficulty gain rate

- Reborn
    - Points gain
        - Based on net gains
    - Increases
        - Increases cash for gold
        - Increases gold gain
        - Increases popularity gain
        - Increases player difficulty gain rate

- Modifiers
    - Amount of workers increase difficulty


- Games
    - Traits
        - Name
        - Price
            - Games have prices to pay once
        - Subscription
            - If the price is a subscription
        - Difficulty
            - Base
            - Growth rate
            - ticks
                - ticks += min(workers*0.75, 1)
            - Difficulty increases with time to simulate anticheat/botting improvements
            - The higher the Difficulty the greater the ban chance
            - If banned reduce gold by (daily gold rate * ban chance) 
                to simulate if a bot gets banned and their gold is deleted
                charge price fee and if not enough to pay reduce workers
        - Cheats
            - Cheats can be toggled off and on
            - Cheats can be developed for increase gold return with an increase to difficulty
        - Rewards:
            - Amount of gold to give per worker per tick
        - Workers:
            - Amount of workers assigned
        - Gold
            - How much gold you have avaliable to sell
        - Popularity
            - How popular the game is, should vary and will impact how much you are able to sell
        
    

