
#ifndef occtwrapper_OCCTWrapper_hpp_
#define occtwrapper_OCCTWrapper_hpp_

#include "rust/cxx.h"

namespace OcctWrapper
{

    bool convert_step_to_stl(rust::String step_file_path, rust::String stl_file_path, rust::f64 chord_error, rust::f64 angle_res);

}; // namespace OcctWrapper

#endif // occtwrapper_OCCTWrapper_hpp_