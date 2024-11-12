// automatically generated by the FlatBuffers compiler, do not modify

package openxml_office.document_2007;

import com.google.flatbuffers.BaseVector;
import com.google.flatbuffers.BooleanVector;
import com.google.flatbuffers.ByteVector;
import com.google.flatbuffers.Constants;
import com.google.flatbuffers.DoubleVector;
import com.google.flatbuffers.FlatBufferBuilder;
import com.google.flatbuffers.FloatVector;
import com.google.flatbuffers.IntVector;
import com.google.flatbuffers.LongVector;
import com.google.flatbuffers.ShortVector;
import com.google.flatbuffers.StringVector;
import com.google.flatbuffers.Struct;
import com.google.flatbuffers.Table;
import com.google.flatbuffers.UnionVector;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

@SuppressWarnings("unused")
public final class DocumentSettingsModel extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_24_3_25(); }
  public static DocumentSettingsModel getRootAsDocumentSettingsModel(ByteBuffer _bb) { return getRootAsDocumentSettingsModel(_bb, new DocumentSettingsModel()); }
  public static DocumentSettingsModel getRootAsDocumentSettingsModel(ByteBuffer _bb, DocumentSettingsModel obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public DocumentSettingsModel __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void startDocumentSettingsModel(FlatBufferBuilder builder) { builder.startTable(0); }
  public static int endDocumentSettingsModel(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public DocumentSettingsModel get(int j) { return get(new DocumentSettingsModel(), j); }
    public DocumentSettingsModel get(DocumentSettingsModel obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}

