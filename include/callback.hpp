#pragma once

#include "cxx-callback/src/callback.rs.h"

void callme(rust::Fn<int(int)> cb);
