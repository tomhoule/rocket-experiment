// Code generated by protoc-gen-go. DO NOT EDIT.
// source: proto/repository.proto

/*
Package repository is a generated protocol buffer package.

It is generated from these files:
	proto/repository.proto

It has these top-level messages:
	GetEditionsParams
	Edition
	Editions
	GetSchemaParams
	EthicsSchema
*/
package repository

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import _ "google.golang.org/genproto/googleapis/api/annotations"

import (
	context "golang.org/x/net/context"
	grpc "google.golang.org/grpc"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

type EthicsSchema_NodeType int32

const (
	EthicsSchema_UNSPECIFIED        EthicsSchema_NodeType = 0
	EthicsSchema_ANONYMOUS_FRAGMENT EthicsSchema_NodeType = 1
	EthicsSchema_ALITER             EthicsSchema_NodeType = 2
	EthicsSchema_APPENDIX           EthicsSchema_NodeType = 3
	EthicsSchema_AXIOMA             EthicsSchema_NodeType = 4
	EthicsSchema_CAPUT              EthicsSchema_NodeType = 5
	EthicsSchema_COROLLARIUM        EthicsSchema_NodeType = 6
	EthicsSchema_DEFINITIO          EthicsSchema_NodeType = 7
	EthicsSchema_DEMONSTRATIO       EthicsSchema_NodeType = 8
	EthicsSchema_EXPLICATIO         EthicsSchema_NodeType = 9
	EthicsSchema_LEMMA              EthicsSchema_NodeType = 11
	EthicsSchema_PARS               EthicsSchema_NodeType = 12
	EthicsSchema_POSTULATUM         EthicsSchema_NodeType = 13
	EthicsSchema_PRAEFATIO          EthicsSchema_NodeType = 14
	EthicsSchema_PROPOSITIO         EthicsSchema_NodeType = 15
	EthicsSchema_SCHOLIUM           EthicsSchema_NodeType = 16
)

var EthicsSchema_NodeType_name = map[int32]string{
	0:  "UNSPECIFIED",
	1:  "ANONYMOUS_FRAGMENT",
	2:  "ALITER",
	3:  "APPENDIX",
	4:  "AXIOMA",
	5:  "CAPUT",
	6:  "COROLLARIUM",
	7:  "DEFINITIO",
	8:  "DEMONSTRATIO",
	9:  "EXPLICATIO",
	11: "LEMMA",
	12: "PARS",
	13: "POSTULATUM",
	14: "PRAEFATIO",
	15: "PROPOSITIO",
	16: "SCHOLIUM",
}
var EthicsSchema_NodeType_value = map[string]int32{
	"UNSPECIFIED":        0,
	"ANONYMOUS_FRAGMENT": 1,
	"ALITER":             2,
	"APPENDIX":           3,
	"AXIOMA":             4,
	"CAPUT":              5,
	"COROLLARIUM":        6,
	"DEFINITIO":          7,
	"DEMONSTRATIO":       8,
	"EXPLICATIO":         9,
	"LEMMA":              11,
	"PARS":               12,
	"POSTULATUM":         13,
	"PRAEFATIO":          14,
	"PROPOSITIO":         15,
	"SCHOLIUM":           16,
}

func (x EthicsSchema_NodeType) String() string {
	return proto.EnumName(EthicsSchema_NodeType_name, int32(x))
}
func (EthicsSchema_NodeType) EnumDescriptor() ([]byte, []int) { return fileDescriptor0, []int{4, 0} }

type GetEditionsParams struct {
	Languages string `protobuf:"bytes,1,opt,name=languages" json:"languages,omitempty"`
}

func (m *GetEditionsParams) Reset()                    { *m = GetEditionsParams{} }
func (m *GetEditionsParams) String() string            { return proto.CompactTextString(m) }
func (*GetEditionsParams) ProtoMessage()               {}
func (*GetEditionsParams) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *GetEditionsParams) GetLanguages() string {
	if m != nil {
		return m.Languages
	}
	return ""
}

type Edition struct {
	Title        string `protobuf:"bytes,1,opt,name=title" json:"title,omitempty"`
	Slug         string `protobuf:"bytes,2,opt,name=slug" json:"slug,omitempty"`
	Editor       string `protobuf:"bytes,3,opt,name=editor" json:"editor,omitempty"`
	Year         int32  `protobuf:"varint,4,opt,name=year" json:"year,omitempty"`
	LanguageCode string `protobuf:"bytes,5,opt,name=language_code,json=languageCode" json:"language_code,omitempty"`
	CreatedAt    string `protobuf:"bytes,6,opt,name=created_at,json=createdAt" json:"created_at,omitempty"`
	UpdatedAt    string `protobuf:"bytes,7,opt,name=updated_at,json=updatedAt" json:"updated_at,omitempty"`
}

func (m *Edition) Reset()                    { *m = Edition{} }
func (m *Edition) String() string            { return proto.CompactTextString(m) }
func (*Edition) ProtoMessage()               {}
func (*Edition) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *Edition) GetTitle() string {
	if m != nil {
		return m.Title
	}
	return ""
}

func (m *Edition) GetSlug() string {
	if m != nil {
		return m.Slug
	}
	return ""
}

func (m *Edition) GetEditor() string {
	if m != nil {
		return m.Editor
	}
	return ""
}

func (m *Edition) GetYear() int32 {
	if m != nil {
		return m.Year
	}
	return 0
}

func (m *Edition) GetLanguageCode() string {
	if m != nil {
		return m.LanguageCode
	}
	return ""
}

func (m *Edition) GetCreatedAt() string {
	if m != nil {
		return m.CreatedAt
	}
	return ""
}

func (m *Edition) GetUpdatedAt() string {
	if m != nil {
		return m.UpdatedAt
	}
	return ""
}

type Editions struct {
	Data []*Edition `protobuf:"bytes,1,rep,name=data" json:"data,omitempty"`
}

func (m *Editions) Reset()                    { *m = Editions{} }
func (m *Editions) String() string            { return proto.CompactTextString(m) }
func (*Editions) ProtoMessage()               {}
func (*Editions) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *Editions) GetData() []*Edition {
	if m != nil {
		return m.Data
	}
	return nil
}

type GetSchemaParams struct {
}

func (m *GetSchemaParams) Reset()                    { *m = GetSchemaParams{} }
func (m *GetSchemaParams) String() string            { return proto.CompactTextString(m) }
func (*GetSchemaParams) ProtoMessage()               {}
func (*GetSchemaParams) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{3} }

type EthicsSchema struct {
	Parts []*EthicsSchema_Node `protobuf:"bytes,1,rep,name=parts" json:"parts,omitempty"`
}

func (m *EthicsSchema) Reset()                    { *m = EthicsSchema{} }
func (m *EthicsSchema) String() string            { return proto.CompactTextString(m) }
func (*EthicsSchema) ProtoMessage()               {}
func (*EthicsSchema) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{4} }

func (m *EthicsSchema) GetParts() []*EthicsSchema_Node {
	if m != nil {
		return m.Parts
	}
	return nil
}

type EthicsSchema_Node struct {
	NodeType EthicsSchema_NodeType `protobuf:"varint,1,opt,name=node_type,json=nodeType,enum=repository.EthicsSchema_NodeType" json:"node_type,omitempty"`
	// Types that are valid to be assigned to Identifier:
	//	*EthicsSchema_Node_Num
	//	*EthicsSchema_Node_Title
	Identifier isEthicsSchema_Node_Identifier `protobuf_oneof:"identifier"`
	Children   []*EthicsSchema_Node           `protobuf:"bytes,4,rep,name=children" json:"children,omitempty"`
}

func (m *EthicsSchema_Node) Reset()                    { *m = EthicsSchema_Node{} }
func (m *EthicsSchema_Node) String() string            { return proto.CompactTextString(m) }
func (*EthicsSchema_Node) ProtoMessage()               {}
func (*EthicsSchema_Node) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{4, 0} }

type isEthicsSchema_Node_Identifier interface {
	isEthicsSchema_Node_Identifier()
}

type EthicsSchema_Node_Num struct {
	Num int32 `protobuf:"varint,2,opt,name=num,oneof"`
}
type EthicsSchema_Node_Title struct {
	Title string `protobuf:"bytes,3,opt,name=title,oneof"`
}

func (*EthicsSchema_Node_Num) isEthicsSchema_Node_Identifier()   {}
func (*EthicsSchema_Node_Title) isEthicsSchema_Node_Identifier() {}

func (m *EthicsSchema_Node) GetIdentifier() isEthicsSchema_Node_Identifier {
	if m != nil {
		return m.Identifier
	}
	return nil
}

func (m *EthicsSchema_Node) GetNodeType() EthicsSchema_NodeType {
	if m != nil {
		return m.NodeType
	}
	return EthicsSchema_UNSPECIFIED
}

func (m *EthicsSchema_Node) GetNum() int32 {
	if x, ok := m.GetIdentifier().(*EthicsSchema_Node_Num); ok {
		return x.Num
	}
	return 0
}

func (m *EthicsSchema_Node) GetTitle() string {
	if x, ok := m.GetIdentifier().(*EthicsSchema_Node_Title); ok {
		return x.Title
	}
	return ""
}

func (m *EthicsSchema_Node) GetChildren() []*EthicsSchema_Node {
	if m != nil {
		return m.Children
	}
	return nil
}

// XXX_OneofFuncs is for the internal use of the proto package.
func (*EthicsSchema_Node) XXX_OneofFuncs() (func(msg proto.Message, b *proto.Buffer) error, func(msg proto.Message, tag, wire int, b *proto.Buffer) (bool, error), func(msg proto.Message) (n int), []interface{}) {
	return _EthicsSchema_Node_OneofMarshaler, _EthicsSchema_Node_OneofUnmarshaler, _EthicsSchema_Node_OneofSizer, []interface{}{
		(*EthicsSchema_Node_Num)(nil),
		(*EthicsSchema_Node_Title)(nil),
	}
}

func _EthicsSchema_Node_OneofMarshaler(msg proto.Message, b *proto.Buffer) error {
	m := msg.(*EthicsSchema_Node)
	// identifier
	switch x := m.Identifier.(type) {
	case *EthicsSchema_Node_Num:
		b.EncodeVarint(2<<3 | proto.WireVarint)
		b.EncodeVarint(uint64(x.Num))
	case *EthicsSchema_Node_Title:
		b.EncodeVarint(3<<3 | proto.WireBytes)
		b.EncodeStringBytes(x.Title)
	case nil:
	default:
		return fmt.Errorf("EthicsSchema_Node.Identifier has unexpected type %T", x)
	}
	return nil
}

func _EthicsSchema_Node_OneofUnmarshaler(msg proto.Message, tag, wire int, b *proto.Buffer) (bool, error) {
	m := msg.(*EthicsSchema_Node)
	switch tag {
	case 2: // identifier.num
		if wire != proto.WireVarint {
			return true, proto.ErrInternalBadWireType
		}
		x, err := b.DecodeVarint()
		m.Identifier = &EthicsSchema_Node_Num{int32(x)}
		return true, err
	case 3: // identifier.title
		if wire != proto.WireBytes {
			return true, proto.ErrInternalBadWireType
		}
		x, err := b.DecodeStringBytes()
		m.Identifier = &EthicsSchema_Node_Title{x}
		return true, err
	default:
		return false, nil
	}
}

func _EthicsSchema_Node_OneofSizer(msg proto.Message) (n int) {
	m := msg.(*EthicsSchema_Node)
	// identifier
	switch x := m.Identifier.(type) {
	case *EthicsSchema_Node_Num:
		n += proto.SizeVarint(2<<3 | proto.WireVarint)
		n += proto.SizeVarint(uint64(x.Num))
	case *EthicsSchema_Node_Title:
		n += proto.SizeVarint(3<<3 | proto.WireBytes)
		n += proto.SizeVarint(uint64(len(x.Title)))
		n += len(x.Title)
	case nil:
	default:
		panic(fmt.Sprintf("proto: unexpected type %T in oneof", x))
	}
	return n
}

func init() {
	proto.RegisterType((*GetEditionsParams)(nil), "repository.GetEditionsParams")
	proto.RegisterType((*Edition)(nil), "repository.Edition")
	proto.RegisterType((*Editions)(nil), "repository.Editions")
	proto.RegisterType((*GetSchemaParams)(nil), "repository.GetSchemaParams")
	proto.RegisterType((*EthicsSchema)(nil), "repository.EthicsSchema")
	proto.RegisterType((*EthicsSchema_Node)(nil), "repository.EthicsSchema.Node")
	proto.RegisterEnum("repository.EthicsSchema_NodeType", EthicsSchema_NodeType_name, EthicsSchema_NodeType_value)
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for EthicsRepository service

type EthicsRepositoryClient interface {
	GetSchema(ctx context.Context, in *GetSchemaParams, opts ...grpc.CallOption) (*EthicsSchema, error)
	GetEditions(ctx context.Context, in *GetEditionsParams, opts ...grpc.CallOption) (*Editions, error)
}

type ethicsRepositoryClient struct {
	cc *grpc.ClientConn
}

func NewEthicsRepositoryClient(cc *grpc.ClientConn) EthicsRepositoryClient {
	return &ethicsRepositoryClient{cc}
}

func (c *ethicsRepositoryClient) GetSchema(ctx context.Context, in *GetSchemaParams, opts ...grpc.CallOption) (*EthicsSchema, error) {
	out := new(EthicsSchema)
	err := grpc.Invoke(ctx, "/repository.EthicsRepository/GetSchema", in, out, c.cc, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *ethicsRepositoryClient) GetEditions(ctx context.Context, in *GetEditionsParams, opts ...grpc.CallOption) (*Editions, error) {
	out := new(Editions)
	err := grpc.Invoke(ctx, "/repository.EthicsRepository/GetEditions", in, out, c.cc, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// Server API for EthicsRepository service

type EthicsRepositoryServer interface {
	GetSchema(context.Context, *GetSchemaParams) (*EthicsSchema, error)
	GetEditions(context.Context, *GetEditionsParams) (*Editions, error)
}

func RegisterEthicsRepositoryServer(s *grpc.Server, srv EthicsRepositoryServer) {
	s.RegisterService(&_EthicsRepository_serviceDesc, srv)
}

func _EthicsRepository_GetSchema_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetSchemaParams)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(EthicsRepositoryServer).GetSchema(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/repository.EthicsRepository/GetSchema",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(EthicsRepositoryServer).GetSchema(ctx, req.(*GetSchemaParams))
	}
	return interceptor(ctx, in, info, handler)
}

func _EthicsRepository_GetEditions_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetEditionsParams)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(EthicsRepositoryServer).GetEditions(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/repository.EthicsRepository/GetEditions",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(EthicsRepositoryServer).GetEditions(ctx, req.(*GetEditionsParams))
	}
	return interceptor(ctx, in, info, handler)
}

var _EthicsRepository_serviceDesc = grpc.ServiceDesc{
	ServiceName: "repository.EthicsRepository",
	HandlerType: (*EthicsRepositoryServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetSchema",
			Handler:    _EthicsRepository_GetSchema_Handler,
		},
		{
			MethodName: "GetEditions",
			Handler:    _EthicsRepository_GetEditions_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "proto/repository.proto",
}

func init() { proto.RegisterFile("proto/repository.proto", fileDescriptor0) }

var fileDescriptor0 = []byte{
	// 653 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x8c, 0x54, 0xcd, 0x4e, 0xdb, 0x4c,
	0x14, 0xc5, 0xe4, 0x87, 0xe4, 0x26, 0x80, 0xb9, 0xf0, 0x45, 0xfe, 0x02, 0x48, 0xd4, 0x5d, 0x94,
	0x15, 0x11, 0xb0, 0xea, 0xa6, 0xd2, 0x34, 0x71, 0xc0, 0x52, 0xfc, 0xa3, 0xb1, 0x23, 0xd1, 0x45,
	0x15, 0x4d, 0xe3, 0x69, 0xb0, 0x14, 0xec, 0xc8, 0x9e, 0x54, 0xca, 0xb6, 0xaf, 0xd0, 0xc7, 0xe9,
	0xae, 0x8f, 0xd0, 0xbe, 0x42, 0x1f, 0xa2, 0x8b, 0x2e, 0x2a, 0x8f, 0x6d, 0x08, 0xa8, 0xad, 0xba,
	0x9b, 0x7b, 0xce, 0x99, 0x33, 0xd7, 0x67, 0xee, 0x18, 0x3a, 0x8b, 0x24, 0x16, 0x71, 0x2f, 0xe1,
	0x8b, 0x38, 0x0d, 0x45, 0x9c, 0xac, 0xce, 0x24, 0x80, 0xf0, 0x80, 0x74, 0x8f, 0x66, 0x71, 0x3c,
	0x9b, 0xf3, 0x1e, 0x5b, 0x84, 0x3d, 0x16, 0x45, 0xb1, 0x60, 0x22, 0x8c, 0xa3, 0x34, 0x57, 0xea,
	0xe7, 0xb0, 0x77, 0xc5, 0x85, 0x11, 0x84, 0x12, 0x74, 0x59, 0xc2, 0xee, 0x52, 0x3c, 0x82, 0xe6,
	0x9c, 0x45, 0xb3, 0x25, 0x9b, 0xf1, 0x54, 0x53, 0x4e, 0x94, 0xd3, 0x26, 0x7d, 0x00, 0xf4, 0x2f,
	0x0a, 0x6c, 0x15, 0x1b, 0xf0, 0x00, 0x6a, 0x22, 0x14, 0x73, 0x5e, 0xa8, 0xf2, 0x02, 0x11, 0xaa,
	0xe9, 0x7c, 0x39, 0xd3, 0x36, 0x25, 0x28, 0xd7, 0xd8, 0x81, 0x3a, 0x0f, 0xb2, 0x8e, 0xb4, 0x8a,
	0x44, 0x8b, 0x2a, 0xd3, 0xae, 0x38, 0x4b, 0xb4, 0xea, 0x89, 0x72, 0x5a, 0xa3, 0x72, 0x8d, 0xcf,
	0x61, 0xbb, 0x3c, 0x6e, 0x32, 0x8d, 0x03, 0xae, 0xd5, 0xe4, 0x96, 0x76, 0x09, 0xf6, 0xe3, 0x80,
	0xe3, 0x31, 0xc0, 0x34, 0xe1, 0x4c, 0xf0, 0x60, 0xc2, 0x84, 0x56, 0xcf, 0xbb, 0x2c, 0x10, 0x22,
	0x32, 0x7a, 0xb9, 0x08, 0x4a, 0x7a, 0x2b, 0xa7, 0x0b, 0x84, 0x08, 0xfd, 0x12, 0x1a, 0xe5, 0x47,
	0xe3, 0x0b, 0xa8, 0x06, 0x4c, 0x30, 0x4d, 0x39, 0xa9, 0x9c, 0xb6, 0x2e, 0xf6, 0xcf, 0xd6, 0xe2,
	0x2c, 0x34, 0x54, 0x0a, 0xf4, 0x3d, 0xd8, 0xbd, 0xe2, 0xc2, 0x9b, 0xde, 0xf2, 0x3b, 0x96, 0x47,
	0xa5, 0xff, 0xac, 0x40, 0xdb, 0x10, 0xb7, 0xe1, 0x34, 0xcd, 0x61, 0xbc, 0x84, 0xda, 0x82, 0x25,
	0x22, 0x2d, 0xdc, 0x8e, 0x1f, 0xb9, 0xad, 0x09, 0xcf, 0xec, 0x38, 0xe0, 0x34, 0xd7, 0x76, 0x3f,
	0x2b, 0x50, 0xcd, 0x6a, 0x7c, 0x05, 0xcd, 0x28, 0x0e, 0xf8, 0x44, 0xac, 0x16, 0x79, 0xa6, 0x3b,
	0x17, 0xcf, 0xfe, 0xea, 0xe0, 0xaf, 0x16, 0x9c, 0x36, 0xa2, 0x62, 0x85, 0x08, 0x95, 0x68, 0x79,
	0x27, 0x83, 0xaf, 0x5d, 0x6f, 0xd0, 0xac, 0xc0, 0x4e, 0x79, 0x47, 0x32, 0xf8, 0xeb, 0x8d, 0xf2,
	0x96, 0x5e, 0x42, 0x63, 0x7a, 0x1b, 0xce, 0x83, 0x84, 0x47, 0x5a, 0xf5, 0x5f, 0x9a, 0xbd, 0x97,
	0xbf, 0x6e, 0x03, 0x84, 0x01, 0x8f, 0x44, 0xf8, 0x3e, 0xe4, 0x89, 0xfe, 0x43, 0x81, 0x46, 0xd9,
	0x0b, 0xee, 0x42, 0x6b, 0x6c, 0x7b, 0xae, 0xd1, 0x37, 0x87, 0xa6, 0x31, 0x50, 0x37, 0xb0, 0x03,
	0x48, 0x6c, 0xc7, 0x7e, 0x63, 0x39, 0x63, 0x6f, 0x32, 0xa4, 0xe4, 0xca, 0x32, 0x6c, 0x5f, 0x55,
	0x10, 0xa0, 0x4e, 0x46, 0xa6, 0x6f, 0x50, 0x75, 0x13, 0xdb, 0xd0, 0x20, 0xae, 0x6b, 0xd8, 0x03,
	0xf3, 0x46, 0xad, 0x48, 0xe6, 0xc6, 0x74, 0x2c, 0xa2, 0x56, 0xb1, 0x09, 0xb5, 0x3e, 0x71, 0xc7,
	0xbe, 0x5a, 0xcb, 0x9c, 0xfb, 0x0e, 0x75, 0x46, 0x23, 0x42, 0xcd, 0xb1, 0xa5, 0xd6, 0x71, 0x1b,
	0x9a, 0x03, 0x63, 0x68, 0xda, 0xa6, 0x6f, 0x3a, 0xea, 0x16, 0xaa, 0xd0, 0x1e, 0x18, 0x96, 0x63,
	0x7b, 0x3e, 0x25, 0x19, 0xd2, 0xc0, 0x1d, 0x00, 0xe3, 0xc6, 0x1d, 0x99, 0x7d, 0x59, 0x37, 0x33,
	0xb3, 0x91, 0x61, 0x59, 0x44, 0x6d, 0x61, 0x03, 0xaa, 0x2e, 0xa1, 0x9e, 0xda, 0xce, 0x44, 0xae,
	0xe3, 0xf9, 0xe3, 0x11, 0xf1, 0xc7, 0x96, 0xba, 0x9d, 0xb9, 0xba, 0x94, 0x18, 0x43, 0xb9, 0x67,
	0x47, 0xd2, 0xd4, 0x71, 0x1d, 0x4f, 0x9e, 0xb2, 0x9b, 0xb5, 0xea, 0xf5, 0xaf, 0x9d, 0x51, 0xd6,
	0x82, 0x7a, 0xf1, 0x55, 0x01, 0x35, 0x0f, 0x8a, 0xde, 0x27, 0x87, 0x6f, 0xa1, 0x79, 0x3f, 0x26,
	0x78, 0xb8, 0x9e, 0xe9, 0x93, 0xe9, 0xe9, 0x6a, 0x7f, 0x0a, 0x5c, 0xff, 0xff, 0xe3, 0xb7, 0xef,
	0x9f, 0x36, 0xf7, 0x71, 0xaf, 0xf7, 0xe1, 0xbc, 0xc7, 0x25, 0xd3, 0x4b, 0x73, 0xc7, 0x09, 0xb4,
	0xd6, 0x9e, 0x2c, 0x1e, 0x3f, 0x39, 0xe0, 0xf1, 0x5b, 0xee, 0x1e, 0xfc, 0x66, 0x9c, 0x53, 0xfd,
	0x50, 0xda, 0xff, 0x87, 0xfb, 0x6b, 0xf6, 0xbc, 0x20, 0xdf, 0xd5, 0xe5, 0xaf, 0xe1, 0xf2, 0x57,
	0x00, 0x00, 0x00, 0xff, 0xff, 0xeb, 0xd6, 0x96, 0x4c, 0x5e, 0x04, 0x00, 0x00,
}
