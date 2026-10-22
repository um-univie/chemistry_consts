# Third-party reference data

`src/uff.rs` and `src/mmff.rs` contain the UFF and MMFF94 parameter tables,
lookup conventions and model coefficients ported from RDKit's
`Code/ForceField/UFF/Params.cpp` and `Code/ForceField/MMFF/Params.cpp` and
related force-field sources. They were relocated from Oxidane without
changing numerical values. RDKit distributes this material under BSD 3-Clause.

`src/crippen.rs` contains Wildman-Crippen atomic contributions (1999),
as distributed with RDKit. `src/polar_surface.rs` contains the Ertl
polar-surface-area reference contributions previously held in Oxidane.

The upstream source files also carry these notices:

- UFF parameters: Copyright (C) 2004–2021 Greg Landrum and other RDKit contributors.
- MMFF parameters: Copyright (C) 2013 Paolo Tosco; Copyright (C) 2004–2006
  Rational Discovery LLC.
- Crippen contributions: Copyright (C) 2002–2012 Greg Landrum and Rational
  Discovery LLC.

RDKit's own licence text:

> Copyright (c) 2006-2015, Rational Discovery LLC, Greg Landrum, and Julie
> Penzotti and others. All rights reserved.
>
> Redistribution and use in source and binary forms, with or without
> modification, are permitted provided that the following conditions are met:
>
> * Redistributions of source code must retain the above copyright notice,
>   this list of conditions and the following disclaimer.
> * Redistributions in binary form must reproduce the above copyright notice,
>   this list of conditions and the following disclaimer in the documentation
>   and/or other materials provided with the distribution.
> * Neither the name of the copyright holder nor the names of its
>   contributors may be used to endorse or promote products derived from this
>   software without specific prior written permission.
>
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
> AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
> IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
> ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
> LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
> CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
> SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
> INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
> CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
> ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
> POSSIBILITY OF SUCH DAMAGE.

The UFF parameters and functional form are from Rappe, Casewit, Colwell,
Goddard & Skiff, *J. Am. Chem. Soc.* **1992**, 114, 10024-10035. The MMFF94
atom types are Halgren's, *J. Comput. Chem.* **1996**, 17, 490-519 (MMFF I),
Table III, and the charge model is the same author's MMFF.V, *J. Comput.
Chem.* **1996**, 17, 616-641. The MMFF94 parameters themselves are Merck's,
copyright Merck and Co., Inc., 1994-1996, distributed with RDKit under the
licence above.

## cheq — MIT

The QEq table and hydrogen-hardness coefficient in `src/charge_params.rs`
follow [cheq](https://github.com/caltechmsc/cheq), specifically
`resources/qeq.data.toml` and `solver::implementation::H_CHARGE_DEPENDENCE_FACTOR`.
The model is Rappe & Goddard, *J. Phys. Chem.* **1991**, 95, 3358.

The cheq licence text:

> MIT License
> 
> Copyright (c) 2025 California Institute of Technology, Materials and Process Simulation Center (MSC)
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.
