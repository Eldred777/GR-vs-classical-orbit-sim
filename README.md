# Overview
 A quick little comparison of Newtonian and general relativistic Schwarzschild orbits. The purpose of this project is just to do a little something in Rust to get familiar with some of the basics. This also happens to be my first time using a proper GUI library; at the suggestion of [villuna](https://github.com/villuna), I will attempt to use [raylib-rs](https://github.com/deltaphc/raylib-rs). 

# Background theory 
This will not serve as an appropriate mathematical introduction to either Newtonian orbital mechanics nor general relativity. As a refresher, please refer to Wikipedia: 
- [Newtonian orbital mechanics](https://en.wikipedia.org/wiki/Orbital_mechanics)
- [Schwarzschild metric](https://en.wikipedia.org/wiki/Schwarzschild_metric)
 
 To fix notation, $x$ represents the position of the orbiting body, with components $x=(r,\phi,\theta)$ in cylindrical coordinates. Notice that we are using the physics convention, with $\phi$ the azimuthal angle and $\theta$ the polar angle. 
 
 Throughout we assume $c=G=1$ for convenience, and that we are on the plane so that the polar angle is given by $\theta=\pi/2$. 
 Further, we denote derivatives with respect to (coordinate) time as dots, $\dot{x}$. 

 **Important note: as written, this project simulates the path of the object with respect to coordinate time, as opposed to proper time. It also currently fails to account for the coordinate singularity of the Schwarzschild metric at the event horizon.**

 ## Newtonian equations of motion
 Recall we have the usual $F= ma = m \ddot{r}$ so with the law of gravitation
 ```math
 F = - \frac{Mm}{r^2}
 ```
 with $M$ the mass of the central body (think black hole or star), $m$ the mass 
 we essentially have everything we need! However we in fact wish to use spherical coordinates so we shall do a little bit more work and pass to the Lagrangian formulation. We introduce the Lagrangian $$L = T - V$$ where $T$ is the kinetic energy and $V$ is the potential energy of the system. Thus we have 
 ```math
 L = \frac{m\dot{x}^2}{2} + \frac{Mm}{r} = \frac{m}{2} \left( \dot{r}^2 + r^2 \dot{\phi}^2 \right) + \frac{Mm}{r}. 
 ```
 
 Then applying the [variational principle](https://en.wikipedia.org/wiki/Variational_principle) we have the [Euler-Lagrange](https://en.wikipedia.org/wiki/Euler%E2%80%93Lagrange_equation) 
 ```math
 0 = \frac{\partial L}{\partial q} - \frac{d}{dt}\frac{\partial L}{\partial \dot{q}} . 
 ```
 This yields the two equations of motion, 
 ```math
 \ddot{r} = r \dot{\phi}^2 - \frac{M}{r^2}, 
 ```
 ```math
 \ddot{\phi} = - \frac{2 \dot{r}\dot{\phi}}{r}. 
 ```

 ## Schwarzschild equations of motion
 Recall that the Schwarzschild metric with fixed $\theta = \pi/2$ is given by 
 ```math
 ds^2 = - \left( 1 - \frac{2M}{r} \right) dt^2 + \left( 1 - \frac{2M}{r} \right)^{-1} dr^2 + r^2 d\phi^2 .
 ```
 Objects fall along geodesics in general relativity, for which we have the equations in terms of proper time (here denoted by primes)
 ```math
 t'' = - \frac{2M}{ r (r - 2M) } r' t' ,
 ```
 ```math
 r''  = - \frac{M (r-2M)}{r^3} t'^2 + \frac{M}{r (r - 2M)} r'^2 + (r-2M) (\phi')^2 ,
 ```
 ```math
 \phi''  = - \frac{2 r' \phi'}{r} .
 ```
 Using the chain rule, the above equations become (with care taken for the conversion $x'' \to \ddot{x}$)
 ```math
 \ddot{r} = - \frac{M (r-2M)}{r^3} + \frac{3 M}{r (r - 2M)} \dot{r}^2 + (r-2M) \dot{\phi}^2 ,
 ```
 ```math
 \ddot{\phi} = \frac{2M}{r (r-2M)} \dot{r} \dot{\phi} - \frac{2 \dot{r} \dot{\phi}}{r} .
 ```

 ## Computational techniques 
 We now say introduce the techniques used to actually use these differential equations in our program. 
 First, we convert each differential equation $\ddot{x} = f(x, \dot{x})$ into two differential equations by setting $y = \dot{x}$ and so $\dot{x} = y,$ and $\dot{y} = f(x,y)$. 
 Note explicitly that all differential equations here are autonomous (no dependence on the "time" parameter).

 We can then use first order solvers on the systems, for which we implement the [Euler method](https://en.wikipedia.org/wiki/Euler_method) and [Runge-Katta methods](https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods) with 4 iterations (RK4). 
