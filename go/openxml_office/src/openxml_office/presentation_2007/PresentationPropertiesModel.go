// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package presentation_2007

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type PresentationPropertiesModel struct {
	_tab flatbuffers.Table
}

func GetRootAsPresentationPropertiesModel(buf []byte, offset flatbuffers.UOffsetT) *PresentationPropertiesModel {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &PresentationPropertiesModel{}
	x.Init(buf, n+offset)
	return x
}

func FinishPresentationPropertiesModelBuffer(builder *flatbuffers.Builder, offset flatbuffers.UOffsetT) {
	builder.Finish(offset)
}

func GetSizePrefixedRootAsPresentationPropertiesModel(buf []byte, offset flatbuffers.UOffsetT) *PresentationPropertiesModel {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &PresentationPropertiesModel{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func FinishSizePrefixedPresentationPropertiesModelBuffer(builder *flatbuffers.Builder, offset flatbuffers.UOffsetT) {
	builder.FinishSizePrefixed(offset)
}

func (rcv *PresentationPropertiesModel) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *PresentationPropertiesModel) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *PresentationPropertiesModel) IsInMemory() bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetBool(o + rcv._tab.Pos)
	}
	return false
}

func (rcv *PresentationPropertiesModel) MutateIsInMemory(n bool) bool {
	return rcv._tab.MutateBoolSlot(4, n)
}

func PresentationPropertiesModelStart(builder *flatbuffers.Builder) {
	builder.StartObject(1)
}
func PresentationPropertiesModelAddIsInMemory(builder *flatbuffers.Builder, isInMemory bool) {
	builder.PrependBoolSlot(0, isInMemory, false)
}
func PresentationPropertiesModelEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
