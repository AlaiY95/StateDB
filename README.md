# StateDB

StateDB is a high-performance REVM database implementation designed for direct integration with the RETH database. It provides real-time access to the latest canonical chain state while supporting custom state modifications.

# Use Case

Direct Database Access: Eliminates RPC/IPC overhead by connecting directly to the RETH database
Real-time State: Always operates on the latest canonical chain state
Custom State Management: Supports insertion of custom accounts and storage modifications
High Performance: Significantly faster than traditional RPC/IPC methods for state access

# Inspiration
StateDB has been developed to solve a specific challenge in my arbitrage bot implementation. The bot requires accurate price simulations through a custom quoter contract while maintaining real-time chain state awareness. 

The development of StateDB was driven by the need for a database solution that could allow me to insert contract code and state information while also being able to operate using the most up to date state for proper quptes.

Traditional solutions like CacheDB and AlloyDB either lacked state insertion capabilities or couldn't maintain current chain state, making accurate price simulations impossible. StateDB bridges this gap by providing both custom state manipulation and real-time chain data access in a single implementation.

The ability to both inject custom state (contract code, token balances, approvals) while simultaneously accessing current chain state makes StateDB particularly effective for arbitrage simulations and other DeFi applications requiring state manipulation with real-time data.
