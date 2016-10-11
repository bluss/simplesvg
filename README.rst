simplesvg
=========

Very simple drawing/diagramming library with svg output.

Please read the `API documentation here`__

__ https://docs.rs/simplesvg/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/simplesvg.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/simplesvg

.. |crates| image:: http://meritbadge.herokuapp.com/simplesvg
.. _crates: https://crates.io/crates/simplesvg


Recent Changes
--------------

- 0.4.0

  - Fix circle and ellipse drawing
  - Add ability to use "none" as color

- 0.3.1

  - Update doc link

- 0.3.0

  - Transformations are now an ordered sequence.
    The sequence .rotate(x).translate(x, y) behaves as if the transformations
    are nested like ``<g transform="translate(x, y)"> <g transform="rotate(x)">``.

License
=======

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.


