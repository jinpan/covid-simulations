---
layout: page
sidebar_title: SIM&#58; Shopping with Masks
title: Shopping with Masks
permalink: /shopping_with_masks/
og_image: /shopping_with_masks/og_image.png
---

<script src="./shopping_with_masks.bundle.js"></script>

### 2020-05-22 | Jin Pan | <a href="https://twitter.com/jinpan20?ref_src=twsrc%5Etfw" class="twitter-follow-button" data-show-count="false">Follow @jinpan20</a><script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

*Before we dive in, you may want to check out the [Intro to SIR/SEIR simulations post](/intro)*.

The [CDC recommends wearing masks outdoors](https://www.cdc.gov/coronavirus/2019-ncov/prevent-getting-sick/diy-cloth-face-coverings.html).
You may wonder, how effective are masks?  What's the difference between wearing a N95 mask vs a cloth one?  We will
explore answers to these questions and more.

> How does our usage of masks affect the spread of Covid-19?

Let's run some
<span style="background-color:#B8F7BF">S</span><span style="background-color:#C7BA29">E</span><span style="background-color:#EB6383">I</span><span style="background-color:#C8C8C8">R</span>
 simulations built off of our [Shopping Solo](/shopping_solo) simulations.
People marked with a black border are wearing a cloth mask at all times: those wearing masks emit 80% fewer viral
particles vs those who don't, but inhale the same amount.  This mask behavior
is based on [CDC guidance](https://www.cdc.gov/coronavirus/2019-ncov/prevent-getting-sick/cloth-face-cover-faq.html),
and all parameters are at this footnote[^shopping_parameters].

[^shopping_parameters]: Cloth Mask Simulation parameters:
    * At 1x speed, each second is divided into 60 ticks.  At 2x speed, each second into 120 ticks, and so on.
    * The map is a 600 x 400 grid.
    * There are 54 people and 54 households, one person per household.
    * Within households and stores, people will move at 1 unit per tick, and bounce off of walls.
    * On roads, people will move at 3 units per tick and take the shortest path from their household to the store
      and vice versa.
    * Households randomly start with 150-450 supplies, and consume 1 supply per tick.  This is independent of how many
      people are in the household at that time.
    * Households go shopping when the supply level hits 0.  However, the supply level will continue
    dropping by 1 per tick and go negative.
    * People will spend 600 ticks at the store, not including time to travel to/from
    the store.  Travel time depends on household distance to the store.
    * People will buy 1800 supplies each time they go to the store.
    * Initially, two randomly chosen people are infected.
    * Infectious people without a mask will exhale 1 particle in a circle of radius 9 around them on the grid, once per
      tick
    * Infectious people with a mask will exhale 0.2 particles in a circle of radius 9 around them on the grid, once per
      tick
    * People only inhale particles for the grid point that they are on.
    * When a susceptible person inhales N particles, they have a N * 0.04% chance of becoming exposed during on that
      tick.  They do not accumulate inhaled particles - each tick is independent of the previous.  This is independent
      of whether they have a mask.
    * When initially exposed, people stay exposed for 900 ticks and become infectious for 2700 ticks before recovering.
    * Each tick, the number of particles at each location on the grid decays by 5.5%
    * The astute reader will notice that the simulation parameters are not identical to the shopping solo parameters.  These
    parameters were calibrated to illustrate the difference that mask wearing could have in a scenario where mask wearing
    affects the trajectory of the infection.  A future post will combine all the shopping simulations together, allowing
    us to compare the relative impact of the actions.
    
You can configure the percentage of people who wear masks and see the impact on disease
spread.

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
  <span>% of people with Cloth Masks:
    <button class="particle_shopper0-pct-mask" data-pct="0">0%</button>
    <button class="particle_shopper0-pct-mask" data-pct="25" style="font-weight: bold" disabled>25%</button>
    <button class="particle_shopper0-pct-mask" data-pct="50">50%</button>
    <button class="particle_shopper0-pct-mask" data-pct="75">75%</button>
    <button class="particle_shopper0-pct-mask" data-pct="100">100%</button>
  </span>

  <br>
  <div
          id="particle_shopper0-uplot"
          style="border: solid; border-width: thin; display: inline-block; width: 100%"
  ></div>

  <canvas
          id="particle_shopper0-canvas"
          width="600" height="400"
          style="border:1px solid #000000; width: 100%">
  </canvas>
</div>

Note how infected people with a mask have a smaller cloud of particles around them, vs those without a mask.

<hr>

## Cloth Mask Data Analysis

Running this simulation 10,000x, there is a clear relationship between mask wearing and
infection rates: **mask wearing reduces infection rates**.

<div
    style="border: solid; border-width: thin; display: inline-block; width: 100%"
>
    <div id="infection_rate_vs_pct_mask"></div>

    <div style="padding: 1em">
The shaded green area contains the average 50% of outcomes across all the simulations.
The solid green line is the median outcome, and the thin dotted line is the best fit over
the entire graph.  The thick gray line from 40-60% is the best fit over that narrower range.
    </div>
</div>

Looking more closely at the data, there is an interesting second-order effect: a backwards "S-curve".

**When about half of people are wearing masks, each additional mask wearer can prevent the most number of infections.**

Intuitively, this makes sense:
* When few people wear masks, the infection spreads rapidly.  Those who wear masks still have a high
chance of getting infected since regular masks do not directly protect the wearer.
* When many people wear masks, the infection rates are much lower and unmasked people benefit from "herd immunity".
Additionally, if an unmasked person does get infected and transmits the infection, those who are serially infected
are wearing masks and won't further transmit the infection[^but_still_wear_your_mask].
* In the middle is where our actions are most effective; each additional mask wearer can reduce
the number of total infections by 1.7 (slope of the best fit line from 40 to 60%).

[^but_still_wear_your_mask]:
    If your community has a high rate of mask wearers, you should still wear masks!  The more people we see wearing
    masks, the more normal we will consider masks to be, and the more we will wear them.

<hr>
## What about N95 Masks?

Unlike ordinary masks, N95 masks reduce inhaled particles by 95%[^n95_effectiveness], theoretically reducing the
wearer's infection risk.  They are uncomfortable to wear, currently in short supply, and the
[CDC recommends saving them for healthcare workers](https://www.cdc.gov/coronavirus/2019-ncov/prevent-getting-sick/cloth-face-cover-faq.html).
But in simulation they are neither in short supply nor uncomfortable, so we can ask

[^n95_effectiveness]: Although for some smaller virons, they may filter out less than 95%: 
    [https://pubmed.ncbi.nlm.nih.gov/16490606/](https://pubmed.ncbi.nlm.nih.gov/16490606/).

> How do N95 masks compare against regular masks?

People marked with a thick black border are wearing a N95 mask; no border, no mask.
People with N95 masks inhale fewer viral particles than those who don't, and all parameters are at this
footnote[^shopping_n95_parameters].

[^shopping_n95_parameters]: N95 Mask Simulation Parameters:
    * These parameters are basically the same as the above parameters, but have some minor differences in how masks work
    * People without a mask:
        * If infectious, they will emit 1 particle per tick in a radius of 1 around them.
        * They will inhale all the particles they are directly standing on.
    * People with a regular mask:
        * If infectious, they will emit 0.2 particles per tick in a radius of 1 around them.
        * They will inhale all the particles they are directly standing on.
    * People with a N95 mask:
        * If infectious, they will emit 0.2 particles per tick in a radius of 1 around them.
        * They will inhale 20% of the particles they are directly standing on.  This was deliberately chosen as >5%,
        to allow for errors in fit and fomite transmission.

<div>
  <button id="particle_shopper1-start" style="width: 4em">Start</button>
  <button id="particle_shopper1-reset">Reset</button>
  <span>Speed:
    <button class="particle_shopper1-speed" data-speed="1" style="font-weight: bold" disabled>1x</button>
    <button class="particle_shopper1-speed" data-speed="2">2x</button>
    <button class="particle_shopper1-speed" data-speed="4">4x</button>
    <button class="particle_shopper1-speed" data-speed="8">8x</button>
    <button class="particle_shopper1-speed" data-speed="16">16x</button>
    <button class="particle_shopper1-speed" data-speed="32">32x</button>
  </span>
  <br><span>% N95 Masks:
    <button class="particle_shopper1-pct-n95-mask" data-pct="0">0%</button>
    <button class="particle_shopper1-pct-n95-mask" data-pct="25" style="font-weight: bold" disabled>25%</button>
    <button class="particle_shopper1-pct-n95-mask" data-pct="50">50%</button>
    <button class="particle_shopper1-pct-n95-mask" data-pct="75">75%</button>
    <button class="particle_shopper1-pct-n95-mask" data-pct="100">100%</button>
  </span>

  <br>
  <div
          id="particle_shopper1-uplot"
          style="border: solid; border-width: thin; display: inline-block; width: 100%"
  ></div>

  <canvas
          id="particle_shopper1-canvas"
          width="600" height="400"
          style="border:1px solid #000000; width: 100%">
  </canvas>
</div>
<hr>

## N95 Mask Data Analysis

It is no surprise that N95 masks are more effective than cloth masks, but just how much more effective are they?

<div
    style="border: solid; border-width: thin; display: inline-block"
>
    <div id="infection_rate_vs_pct_n95_mask"></div>

    <div style="padding: 1em">
The blue area represents the average 50% of outcomes as we vary the rate of N95 mask usage.
The green area is the above graph of cloth mask usage overlaid for comparison.
    </div>
</div>

Both types of masks can control the spread with a critical mass of wearers.  The critical mass for N95 masks is lower
than that for regular masks, but not as large as you might imagine. For a max tolerable infection rate, we only need
10-15% more adoption of regular masks vs N95 masks to stay under that limit.

| To keep infection rate under: | % of cloth mask wearers required | % of N95 mask wearers required |
| 50% | 30% | 40% |
| 33% | 35% | 50% |
| 20% | 45% | 55% |

**For each max tolerable infection rate, we only need modestly more adoption of cloth masks vs N95 masks to stay under
that rate.**

Given the shortage of N95 masks, aiming for a modestly higher rate of regular mask usage is more practical policy.

<hr>

## How does wearing a mask affect _my_ risk of getting infected?

A cloth mask does not offer direct protection against inhaling viral particles[^but_maybe_they_help];  for
a N95 mask, it depends.

[^but_maybe_they_help]: One of the primary transmission vectors of the virus is from touching some viral particles, and
    then touching your face.  Mask wearing can reduce the amount of face touching, which provides some protection
    against the virus, but that is not captured in this simulation.

<div
        style="border: solid; border-width: thin; display: inline-block"
>
  <div id="infection_rate_by_mask_type"></div>

  <div style="padding: 1em">
    The red area represents the average 50% infection rates for people who don't wear any mask at all; blue area
    represents the risk for those wearing N95 masks.
  </div>
</div>

When there is a low percentage of people wearing masks, wearing a N95 mask can significantly decrease your infection
risk[^about_that_shortage].  This advantage fades as more people wear masks and the riskiness of the environment falls, which underscores
a key takeaway from our [Shopping Solo](/shopping_solo) simulations.

**Our decisions matter more when our community is more at risk.**

[^about_that_shortage]: I debated internally about whether it is ethical to publish this, since there is a real shortage
    of N95 masks.  In the end, I'm publishing these findings because the advantage of N95 masks is strongest when only
    few people are wearing masks at all.  If acted on, this published finding would only impact a few masks.  Plus, I
    see people at my local Costco (which has 80+% mask usage) with N95 masks, so this may convince them that they're
    not that necessary.  And orthogonally, not publishing this would be lying by omission.

<hr>

## Future work

The best way to combat this virus is to make data-driven policies and decisions.
High quality simulations offer a fast and safe way to estimate the risk of our actions.

That being said, the virus modeled above is not Covid-19 the dots are not shopping the same way
we shop: we touch shopping carts, swipe credit cards, maintain distance in stores, and so on.
Researchers are discovering more about Covid-19 every day, about how it spreads, symptoms it produces,
how to treat it, and so on.

Future work will focus on incorporating the latest research and simulating more realistic human
behavior. These simulations are all [open source](https://www.github.com/jinpan/covid-simulations/).
You can reach out to me privately at `covid-contact@simrnd.com` or on Twitter.

Please share these simulations if you found them informative - as the above data shows, **we all
need to work together to control the spread of the virus**.

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">I built some <a href="https://twitter.com/hashtag/Coronavirus?src=hash&amp;ref_src=twsrc%5Etfw">#Coronavirus</a> simulations, exploring the impact of wearing cloth vs N95 masks. We should all wear masks outdoors, but there is an inflection point where mask wearing is super effective.<a href="https://t.co/AgfV48ghVV">https://t.co/AgfV48ghVV</a> <a href="https://t.co/cpbYBUikHJ">pic.twitter.com/cpbYBUikHJ</a></p>&mdash; Jin Pan (@JinPan20) <a href="https://twitter.com/JinPan20/status/1263906007547174912?ref_src=twsrc%5Etfw">May 22, 2020</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
<a href="https://twitter.com/jinpan20?ref_src=twsrc%5Etfw" class="twitter-follow-button" data-show-count="false">Follow @jinpan20</a><script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

<hr>
<iframe
    src="https://docs.google.com/forms/d/e/1FAIpQLSfHi7RGMyJwAixM_LdcokCgbgpcx6a7EG6LH4bBPh1rUYv-Cg/viewform?embedded=true"
    width="640" height="510" frameborder="0" marginheight="0" marginwidth="0">
    Feedback form
</iframe>

<hr>
### Footnotes:
