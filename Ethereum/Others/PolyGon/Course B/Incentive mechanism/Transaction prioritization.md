
### Effective percentage

The last parameter called the  EffectivePercentage  is used to measure the unused portion of the user’s signed gas price.

In order to calculate the  EffectivePercentage, one option is to consider pricing resources based on the number of consumed counters within the Polygon zkEVM’s proving system.

However, understanding this metric can be challenging for users because stating the efficiency through counters is a bit complicated.

In favor of better UX, a formula involving gas is applied as it is more user-friendly.

The primary objective is to compute  EffectivePercentage  exclusively using gas, while allowing users to prioritize their transactions through the use of gas price, without the need for complex considerations such as used ROM counters.