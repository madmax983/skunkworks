# ⚛️ Genesis: The Routing Market

> "In the internet of things, every packet pays its way."

**The Routing Market** simulates a network where bandwidth is a scarce commodity priced by congestion.

## The Concept

This experiment smashes together **Market Microstructure** and **Network Packet Routing**.
Instead of FIFO queues or simple QoS, this network operates on a **Congestion Pricing** model.

- **Packets (Agents)**: Have a destination and a budget. They bid for routing service.
- **Routers (Market Makers)**: Charge a dynamic toll based on their queue depth (supply/demand).
- **Emergent Behavior**:
    - **Highways**: Low-cost, high-bandwidth links become popular until they congest.
    - **Backroads**: High-latency but cheap paths become viable when highways surge in price.
    - **Packet Drops**: Economic insolvency leads to packet loss (dropped packets are "bankrupt").

## Visuals

- **Nodes**: Routers changing color from Green (Cheap/Idle) to Red (Expensive/Congested).
- **Edges**: Links pulsating with price heat.
- **Packets**: Moving dots traversing the graph.

## Controls

- `q`: Quit
- `b`: **DDOS Burst**. Spawn a massive wave of packets to crash the market.
- `r`: Reset simulation.
