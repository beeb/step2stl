#include "OCCTWrapper.hpp"

#include "STEPControl_Reader.hxx"
#include "BRep_Builder.hxx"
#include "BRepMesh_IncrementalMesh.hxx"
#include "StlAPI_Writer.hxx"

namespace OcctWrapper
{

    bool convert_step_to_stl(rust::String step_file_path, rust::String stl_file_path, rust::f64 chord_error, rust::f64 angle_res, bool binary)
    {
        STEPControl_Reader reader;
        TopoDS_Compound comp;
        BRep_Builder builder;

        builder.MakeCompound(comp);

        IFSelect_ReturnStatus status = reader.ReadFile(step_file_path.c_str());
        if (status != IFSelect_RetDone)
        {
            return false;
        }

        bool failsonly = false;
        reader.PrintCheckLoad(failsonly, IFSelect_ItemsByEntity);

        int nbr = reader.NbRootsForTransfer();
        reader.PrintCheckTransfer(failsonly, IFSelect_ItemsByEntity);

        for (int n = 1; n <= nbr; n++)
        {
            reader.TransferRoot(n);
            int nbs = reader.NbShapes();
            if (nbs > 0)
            {
                for (int i = 1; i <= nbs; i++)
                {
                    TopoDS_Shape shape = reader.Shape(i);
                    BRepMesh_IncrementalMesh mesher(shape, chord_error, false, angle_res, true);
                    builder.Add(comp, shape);
                }
            }
        }

        StlAPI_Writer writer;
        if (binary)
        {
            writer.ASCIIMode() = false;
        }
        bool writeStatus;
        Message_ProgressRange progress = Message_ProgressRange();
        writeStatus = writer.Write(comp, stl_file_path.c_str(), progress);

        return writeStatus;
    }

}; // namespace OcctWrapper