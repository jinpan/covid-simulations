---
layout: page
title: Shopping Solo
permalink: /shopping_solo/
---

<script src="./bootstrap.js"></script>

### 2020-05-15 | Jin Pan | <a href="https://twitter.com/jinpan20?ref_src=twsrc%5Etfw" class="twitter-follow-button" data-show-count="false">Follow @jinpan20</a><script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

Covid-19 has spread across our planet at a rapid pace, infecting
[4.4 million+ people
worldwide](https://www.nytimes.com/interactive/2020/world/coronavirus-maps.html),
with [1.4 million+ cases in the United States as of mid-May
2020](https://www.nytimes.com/interactive/2020/us/coronavirus-us-cases.html).
Until a vaccine is broadly administered, society must continue working together to
control the infection rate.

There are hundreds of small decisions we make each day that
collectively contribute to the infection rate - should I wear a mask?
Should I go shopping solo? Should I wear my lucky socks? Do my
decisions even matter?

While there are definitive answers to some questions ([The CDC recommends
everyone to wear a mask
outdoors](https://www.cdc.gov/coronavirus/2019-ncov/prevent-getting-sick/diy-cloth-face-coverings.html)),
others are harder to answer.  To answer the harder questions, let's
simulate how a hypothetical virus spreads across a virtual population.

First, let's simulate the
[Susceptible-Infectious-Recovered](https://en.wikipedia.org/wiki/Compartmental_models_in_epidemiology#The_SIR_model)
model.  Green circles represent <span style="background-color:
#B8F7BF">susceptible</span> people, red <span style="background-color:
#EB6383">infectious</span>, and gray <span style="background-color:
#C8C8C8">recovered</span>.  A susceptible person who gets too close to an
infectious person will catch the disease.

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
          style="border: solid; border-width: thin; display: inline-block; width:100%"
  ></div>

  <canvas
          id="radius_brownian0-canvas"
          width="600" height="400"
          style="border:1px solid #000000; width:100%">
  </canvas>
</div>

The infected population initially grows rapidly but slows as the susceptible
population shrinks, and eventually the virus runs out of people to infect. In
this rough model, the pandemic is over with 70+% of the population infected.

We can improve this initial simulation by more realistically modeling
1. Virus Spread
1. Human Behavior

We will use the more realistic
[Susceptible-Exposed-Infectious-Recovered](https://en.wikipedia.org/wiki/Compartmental_models_in_epidemiology#The_SEIR_model)
model, which introduces an <span style="background-color: #C7BA29">exposed</span>
state, during which a person does not spread the disease.
[The WHO](https://www.who.int/docs/default-source/coronaviruse/who-china-joint-mission-on-covid-19-final-report.pdf)
estimates that Covid-19's exposed duration is one third the infectious
duration, so we will use that ratio below.

Instead of spreading via contact, the disease will be spread through viral particles:
* Infectious people emit viral particles by breathing, coughing, sneezing, etc
* People continuously inhale viral particles; the more particles they breathe, the
more likely they are to be exposed to the disease
* Viral particles fade over time

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
          style="border: solid; border-width: thin; display: inline-block; width:100%"
  ></div>

  <canvas
          id="particle_brownian0-canvas"
          width="600" height="400"
          style="border:1px solid #000000; width:100%">
  </canvas>
</div>

We are not always trapped in a giant bouncy castle.  Next, let's simulate
more realistic human behavior - shopping.

<hr>

### Shopping Simulation

Let's simulate a tiny community of 108 people split among 54 households (2 people
per household).  Everyone is social distancing, but households must
periodically make trips to the store as they run out of supplies.

Some households (marked `1x`) have a 1x shopper rule that only one person goes
shopping, and other households (marked as `2x`), both people will go out to
shop and stick together in the store. All parameters at this footnote[^shopping_parameters].

[^shopping_parameters]: Shopping simulation parameters:
    * At 1x speed, each second is divided into 60 ticks.  At 2x speed, each second into 120 ticks, and so on.
    * The map is a 600 x 400 grid.
    * Within households and stores, people will move at 1 unit per tick, and bounce off of walls.
    * On roads, people will move at 3 units per tick and take the shortest path from their household to the store
      and vice versa.
    * Households randomly start with 150-450 supplies, and consume 1 supply per tick.  This is independent of how many
      people are in the household at that time.
    * Households go shopping when the supply level hits 0.  However, the supply level will continue
    dropping by 1 per tick and go negative.
    * 1x-shopper households will send out the same person each time they go shopping.
    * At the store, 2x-shopper households stick together.
    * Both types of households will spend 600 ticks at the store, not including time to travel to/from
    the store.  Travel time depends on household distance to the store.
    * Both types of households will buy 1800 supplies from the store and add them to the household supply levels.  This
    is 1800 per household, _not_ per person.
    * Initially, two randomly chosen people are infected.  They could be from the same household with probability 1/103.
    * Infectious people exhale 1 particle in a circle of radius 9 around them on the grid, once per tick
    * People only inhale particles for the grid point that they are on.
    * When a susceptible person inhales N particles, they have a N * 0.013% chance of becoming exposed during on that
      tick.  They do not accumulate inhaled particles - each tick is independent of the previous.
    * When initially exposed, people stay exposed for 900 ticks and become infectious for 2700 ticks before recovering.
    * Each tick, the number of particles at each location on the grid decays by 5.5%


You can configure the percentage of 2x shopper households and observe how that affects
disease spread.

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
  <br>
  <span>Percent of 2x shopper households:
    <button class="particle_shopper0-pct-dual-shopper" data-pct="0">0%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="25">25%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="50" style="font-weight: bold" disabled>50%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="75">75%</button>
    <button class="particle_shopper0-pct-dual-shopper" data-pct="100">100%</button>
  </span>

  <br>
  <div
          id="particle_shopper0-uplot"
          style="border: solid; border-width: thin; display: inline-block; width:100%"
  ></div>

  <canvas
          id="particle_shopper0-canvas"
          width="600" height="400"
          style="border:1px solid #000000; width:100%">
  </canvas>
</div>

<hr>

Running this simulation tens of thousands of times, we see that a 1% increase in
2x-shopper households leads to a ~0.96% increase in people infected at the end of
the simulations.

<div
    style="border: solid; border-width: thin; display: inline-block"
>
    <div id="infection_rate_vs_pct_dual_shopper"></div>

    <div style="padding: 1em">
The shaded gray area contains the average 50% of outcomes across all the simulations.
    </div>
</div>

> What happens if a 1x households sends out a second shopper?

**Each additional shopper infects 1.9 more people:**

* An additional shopper means 1 more 2x shopper household, a `1/54` increase in the number of 2x households
* A 1% increase in number of 2x households leads to a `0.96`% increase in infected people
* There are `108` total people
* `(1/54) * 0.96 * 108 = 1.92`

The intuition behind this is that the additional shopper increases the chances that they get infected, increasing
the chances their household gets infected, increasing the chances that someone in their household infects someone
else, and so on, setting off an infection chain reaction.  In epidemiology
terms, additional shoppers increase the
[effective reproduction number](https://www.healthknowledge.org.uk/public-health-textbook/research-methods/1a-epidemiology/epidemic-theory).

So yes, our decisions absolutely matter and matter beyond ourselves.

<hr>

### How does this affect me?

> How does imposing a 1x-shopper rule in _my_ household affect the risk of
> someone in _my_ household getting the disease?

<div
    style="border: solid; border-width: thin; display: inline-block"
>
    <div id="infection_rate_by_household_type_vs_pct_dual_shopper"></div>

    <div style="padding: 1em">
The green area represents the average 50% of outcomes for
1x-shopper households; the red represents 2x-shopper households.
    </div>
</div>
More details about this chart at this footnote[^my_household_chart_details].

[^my_household_chart_details]: Infection by Household Type vs % 2x Shopper Households chart notes
    * Initially infected households were excluded from this data - their behavior does not cause them to be
      infected.
    * The green `25%` label represents the 25th percentile of infections among 1x shopper households.  The `75%` label
      represents the 75th percentile, and the `50%` label represents the median.
    * There is no data for 2x households at a 0% percentage of 2x households because there are no 2x households.  Same
      for 1x households at 100%.

Our decisions do not exist in a vacuum, and our rate of infection
depends on others within our community. What is interesting here is that
our decisions matter more when our community is more at risk - the gap between
household infection rates increases with the percentage of 2x-shopper households
community.

**Safer decisions matter more when our community is more at risk.**

**The more people we see not taking precautions, the more we need take precautions
ourselves.**

<hr>

### Future work

The best way to combat this virus is to make data-driven policies and decisions.
High quality simulations offer a fast and safe way to estimate the risk of our actions.

That being said, the virus modeled above is not Covid-19 and the tiny community
does not capture real human behavior.  We order delivery services, maintain distance
in stores, self-quarantine if we are sick, and so on.  Researchers are discovering more about
Covid-19 every day, about how it spreads, symptoms it produces, how to treat it, and so on.

Future work will focus on incorporating the latest research and simulating more realistic human
behavior. These simulations are all [open source](https://www.github.com/jinpan/covid-simulations/).
You can reach out to me privately at `covid-contact@simrnd.com` or on Twitter.

Please share these simulations if you found them informative - as the above data shows, **we all
need to work together to control the spread of the virus**.

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">I built some <a href="https://twitter.com/hashtag/coronavirus?src=hash&amp;ref_src=twsrc%5Etfw">#coronavirus</a> simulations, exploring how the way we shop affects the infection rate. Check out the 60fps simulations at <a href="https://t.co/Qa2evarhM4">https://t.co/Qa2evarhM4</a>. <a href="https://t.co/fpGH25QzGM">pic.twitter.com/fpGH25QzGM</a></p>&mdash; Jin Pan (@JinPan20) <a href="https://twitter.com/JinPan20/status/1261462639516909569?ref_src=twsrc%5Etfw">May 16, 2020</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
<a href="https://twitter.com/jinpan20?ref_src=twsrc%5Etfw" class="twitter-follow-button" data-show-count="false">Follow @jinpan20</a><script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

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


### Footnotes
