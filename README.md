# C_boots
Getting estimates of heat capacity ($C$), and associated uncertainties, made easy.
Simply supply a csv of the internal energy ($U$) or enthalpy ($H$), and C_boots with apply bootstrapping to estimate $C_V$ or $C_P$.

## Bootstrapping
Given a sample from a population, we can estimate both the distribution of the sample and the uncertainty in that estimate using a resampling approach known as bootstrapping. 
We begin with the originalsample, which in this case consists of the energies ($E$) obtained from a set of simulations. 
We then repeatedly generate new samples by randomly selecting values from the original sample, with replacement. 
Here, this process is repeated 10,000 times, producing 10,000 resampled sets of the same size as the original sample.
For each resampled set, we calculate the variance in $E$.
The resulting collection of 10,000 variance estimates then allows us to determine both the variance in $E$ and the uncertainty associated with that estimate.

For a more indepth explanation of bootstrapping see: [Bootstrapping](https://en.wikipedia.org/wiki/Bootstrapping_(statistics))
