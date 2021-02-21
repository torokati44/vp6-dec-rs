# License

Most files in this project are licenced depending on how FFmpeg is utilized:
 - If suitable versions of the libraries `libavutil`, `libavcodec`, and
   `libswscale` are found on the system, they will be loaded dynamically at
   runtime, and no code from FFmpeg will be compiled in, so this project can
   be licenced under the MIT license.
 - Otherwise, parts of the LGPL-licenced FFmpeg are compiled into the wrapper
   statically, so the wrapper itself must also be under the GNU Lesser General
   Public License version 2.1 or later (LGPL v2.1+) - read the file
   `COPYING.LGPLv2.1` for details.\
   This option is not available by default, only if the `allow-lgpl` feature
   is enabled.

One exception is `qsort.c` which is always under the MIT license.
