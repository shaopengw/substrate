use support::construct_runtime;

construct_runtime! {
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		Block = Block1,
		UncheckedExtrinsic = Uxt,
	{}
}

fn main() {}
