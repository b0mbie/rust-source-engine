#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum fieldtype_t {
	Void = 0,
	Float,
	String,
	Vector,
	Quaternion,
	Integer,
	Boolean,
	Short,
	Character,
	Color32,
	Embedded,
	Custom,

	ClassPtr,
	EHandle,
	Edict,

	PositionVector,
	Time,
	Tick,
	ModelName,
	SoundName,

	Input,
	Function,

	VMatrix,

	VMatrixWorldSpace,
	Matrix3x4WorldSpace,

	Interval,
	ModelIndex,
	MaterialIndex,

	Vector2d,

	TypeCount,
}
