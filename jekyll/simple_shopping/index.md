---
layout: page
title: Coronavirus and Shopping
permalink: /simple_shopping/
---

<script src="./bootstrap.js"></script>

Covid-19 has been spreading across our planet at a rapid pace, and has infected
more than [4.3 million people
worldwide](https://www.nytimes.com/interactive/2020/world/coronavirus-maps.html),
with more than [1.3 million cases in the United States as of mid-May
2020](https://www.nytimes.com/interactive/2020/us/coronavirus-us-cases.html).
Until a vaccine is broadly administered, society must work together to control
the infection rate.

There are hundreds of small questions we must all answer each day that
collectively contribute to the infection - should I wear a mask? How far away
should I keep from others? How often should I go shopping? Should I go
shopping alone? Should I wear my lucky socks? Do my decisions even matter?

While there are definitive answers to some questions ([The CDC recommends
everyone to wear a mask
outdoors](https://www.cdc.gov/coronavirus/2019-ncov/prevent-getting-sick/diy-cloth-face-coverings.html)),
others are harder to answer.  To answer some of the harder questions, let's look
at some simulations of how a hypothetical virus spreads across a virtual
population and how individual actions affect the spread, both on a societal and
household level.

First, let's simulate the
[Susceptible-Infectious-Recovered](https://en.wikipedia.org/wiki/Compartmental_models_in_epidemiology#The_SIR_model)
model.  Green circles represent susceptible people, red circles infectious, and
gray circles recovered.  If a susceptible person gets too close to an
infectious person, they will catch the disease.

<div>
  <button id="radius_brownian0-start" style="width: 4em">Start</button>
  <button id="radius_brownian0-reset">Reset</button>
  <span>Speed:
    <button class="radius_brownian0-speed" data-speed="1" style="font-weight: bold" disabled>1x</button>
    <button class="radius_brownian0-speed" data-speed="2">2x</button>
    <button class="radius_brownian0-speed" data-speed="4">4x</button>
    <button class="radius_brownian0-speed" data-speed="8">8x</button>
  </span>
  <br>
  <div
          id="radius_brownian0-uplot"
          style="border: solid; border-width: thin; display: inline-block"
  ></div>

  <canvas
          id="radius_brownian0-canvas"
          width="600" height="400"
          style="border:1px solid #000000;">
  </canvas>
</div>

The infected population initially grows rapidly but slows as the susceptible
population shrinks.  Eventually the virus runs out of people to infect, and the
pandemic is over with 70+% of the population infected.

We can improve this initial simulation in two major ways:
1. More realistically model the spread of the virus
1. More realistically model human behavior - we are not always trapped in a giant
bouncy castle

To more realistically model the spread of the virus, we'll add viral particles
to the simulation: infectious people will continuously emit viral particles
(by breathing, coughing, sneezing, etc).  People will also continuously inhale
viral particles, and the more particles they breathe in, the more likely they
are to contract the disease.

Additionally, let's use a more realistic model of disease spread - the
[Susceptible-Exposed-Infectious-Recovered](https://en.wikipedia.org/wiki/Compartmental_models_in_epidemiology#The_SEIR_model)
model.  This model introduces an exposed state, where exposed individuals are
not immediately infectious.  The WHO [estimates that the Covid-19 exposed
duration is about one third as long as the infectious
duration](https://www.who.int/docs/default-source/coronaviruse/who-china-joint-mission-on-covid-19-final-report.pdf),
so we will use that ratio below.

<div>
  <button id="particle_brownian0-start" style="width: 4em">Start</button>
  <button id="particle_brownian0-reset">Reset</button>
  <span>Speed:
    <button class="particle_brownian0-speed" data-speed="1" style="font-weight: bold" disabled>1x</button>
    <button class="particle_brownian0-speed" data-speed="2">2x</button>
    <button class="particle_brownian0-speed" data-speed="4">4x</button>
    <button class="particle_brownian0-speed" data-speed="8">8x</button>
  </span>

  <br>
  <div
          id="particle_brownian0-uplot"
          style="border: solid; border-width: thin; display: inline-block"
  ></div>

  <canvas
          id="particle_brownian0-canvas"
          width="600" height="400"
          style="border:1px solid #000000;">
  </canvas>
</div>

Now that we have a more realistic model of how the virus spreads, let's look at
simulating more realistic human behavior.

For the next simulation, we have a tiny community of 108 people split among 54
households (2 people per household).  These people are social distancing and
not seeing their friends and neighbors, but must periodically make trips to the
store as their toilet paper and other household supplies run out.

Some households have a single-shopper rule that only one person goes shopping,
and they are marked as "1x".  In other households (marked as "2x"), both people
will go out to shop when supplies run out.

You can configure the percentage of single-shopper households and see how quickly
the disease spreads across the community at various percentages.  This simulation
takes longer to play out, so you may consider increasing the simulation speed.

<div>
  <button id="particle_shopper0-start" style="width: 4em">Start</button>
  <button id="particle_shopper0-reset">Reset</button>
  <span>Speed:
    <button class="particle_shopper0-speed" data-speed="1" style="font-weight: bold" disabled>1x</button>
    <button class="particle_shopper0-speed" data-speed="2">2x</button>
    <button class="particle_shopper0-speed" data-speed="4">4x</button>
    <button class="particle_shopper0-speed" data-speed="8">8x</button>
    <button class="particle_shopper0-speed" data-speed="16">16x</button>
    <button class="particle_shopper0-speed" data-speed="32">32x</button>
  </span>
  <!--TODO: dynamically show this controller-->
  <br>
  <span>Percent of dual shopper households:
    <button class="particle_shopper0-pct-dual-shopper" data-pct="0">0%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="25">25%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="50" style="font-weight: bold" disabled>50%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="75">75%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="100">100%</button>
  </span>

  <br>
  <div
          id="particle_shopper0-uplot"
          style="border: solid; border-width: thin; display: inline-block"
  ></div>

  <canvas
          id="particle_shopper0-canvas"
          width="600" height="400"
          style="border:1px solid #000000;">
  </canvas>
</div>

If we run this simulation many times, we will see that every 1% increase in
dual-shopper households leads to approximately a TODO% increase in people
infected at the end of the simulations.

// TODO: add graph!

We can also look at the data from a household-level perspective and answer the
question: How does imposing a single-shopper rule in my household affect the
risk of someone in my household getting the disease?

// TODO: add graph!

<hr>

While we tuned many parameters of our hypothetical virus simulation to
mimic the rate of spread of covid-19, this hypothetical virus is not covid-19
(and there still remains much to be learned about the exact mechanisms of how
covid-19 spreads) and the simulated circles do not capture real human behavior
(we order grocery delivery services, maintain distance in stores, squeeze many
avocados on the shelf to find the ripest ones, and so on).  So please wrap the
above numbers with generous error bars when applying those judgements in your
daily activities.

On the other hand, building high quality models/simulations and making informed
decisions based on them is the best way to combat this virus as a society.  The
cost of rigorous field studies can often be too expensive (both in terms of
time and infections), so improving these simulations and making policy off of
them may be better than the alternative of waiting for field data.

These simulations are all [open
source](https://jinpan.github.com/covid-simulations/).

## Call for help:
I am a software engineer, not an epidemiologist.  If you are an epidemiologist
(or know of one), please get in touch at `covid-contact@simrnd.com`.  I would
like to build more simulations to model how our behavior affects the disease
spread and want these simulations to be calibrated against everything we know
about covid-19.

<hr>

<iframe
    src="https://docs.google.com/forms/d/e/1FAIpQLScaAb4nP7WCOu7TaKnvmtKayJ81Zcs5BH8kmMBD3-Xf61dHzg/viewform?embedded=true"
    width="640" height="807" frameborder="0" marginheight="0" marginwidth="0">
    Feedback form
</iframe>

<hr>

### Related work

For more simulations, check out
* [Harry Stevens of the Washington Post](https://www.washingtonpost.com/graphics/2020/world/corona-simulator/).
* [3Blue1Brown from YouTube](https://www.youtube.com/watch?v=gxAaO2rsdIs).
