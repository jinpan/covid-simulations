---
layout: page
sidebar_title: SIM&#58; Intro
title: Intro to Simulations
permalink: /intro/
---

<script src="./intro.bundle.js"></script>

### 2020-05-18 | Jin Pan | <a href="https://twitter.com/jinpan20?ref_src=twsrc%5Etfw" class="twitter-follow-button" data-show-count="false">Follow @jinpan20</a><script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

Covid-19 has spread across our planet at a rapid pace, infecting
[4.4 million+ people
worldwide](https://www.nytimes.com/interactive/2020/world/coronavirus-maps.html),
with [1.4 million+ cases in the United States as of mid-May
2020](https://www.nytimes.com/interactive/2020/us/coronavirus-us-cases.html).
Until a vaccine is broadly administered, society must continue working together to
control the infection rate.

There are hundreds of small decisions we make each day that
collectively contribute to the infection rate - should I wear a mask?
Should I go [shopping solo](/shopping_solo)? Should I wear my lucky socks?
Do my decisions even matter?

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
more realistic human behavior - [shopping](/shopping_solo).
