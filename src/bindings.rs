/* automatically generated by rust-bindgen 0.70.1 */

#[doc = " @brief An opaque structure; see astcenc_internal.h for definition."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct astcenc_context {
    _unused: [u8; 0],
}
#[doc = " @brief The call was successful."]
pub const astcenc_error_ASTCENC_SUCCESS: astcenc_error = 0;
#[doc = " @brief The call failed due to low memory, or undersized I/O buffers."]
pub const astcenc_error_ASTCENC_ERR_OUT_OF_MEM: astcenc_error = 1;
#[doc = " @brief The call failed due to the build using fast math."]
pub const astcenc_error_ASTCENC_ERR_BAD_CPU_FLOAT: astcenc_error = 2;
#[doc = " @brief The call failed due to an out-of-spec parameter."]
pub const astcenc_error_ASTCENC_ERR_BAD_PARAM: astcenc_error = 3;
#[doc = " @brief The call failed due to an out-of-spec block size."]
pub const astcenc_error_ASTCENC_ERR_BAD_BLOCK_SIZE: astcenc_error = 4;
#[doc = " @brief The call failed due to an out-of-spec color profile."]
pub const astcenc_error_ASTCENC_ERR_BAD_PROFILE: astcenc_error = 5;
#[doc = " @brief The call failed due to an out-of-spec quality value."]
pub const astcenc_error_ASTCENC_ERR_BAD_QUALITY: astcenc_error = 6;
#[doc = " @brief The call failed due to an out-of-spec component swizzle."]
pub const astcenc_error_ASTCENC_ERR_BAD_SWIZZLE: astcenc_error = 7;
#[doc = " @brief The call failed due to an out-of-spec flag set."]
pub const astcenc_error_ASTCENC_ERR_BAD_FLAGS: astcenc_error = 8;
#[doc = " @brief The call failed due to the context not supporting the operation."]
pub const astcenc_error_ASTCENC_ERR_BAD_CONTEXT: astcenc_error = 9;
#[doc = " @brief The call failed due to unimplemented functionality."]
pub const astcenc_error_ASTCENC_ERR_NOT_IMPLEMENTED: astcenc_error = 10;
#[doc = " @brief The call failed due to an out-of-spec decode mode flag set."]
pub const astcenc_error_ASTCENC_ERR_BAD_DECODE_MODE: astcenc_error = 11;
#[doc = " @brief A codec API error code."]
pub type astcenc_error = ::std::os::raw::c_int;
#[doc = " @brief The LDR sRGB color profile."]
pub const astcenc_profile_ASTCENC_PRF_LDR_SRGB: astcenc_profile = 0;
#[doc = " @brief The LDR linear color profile."]
pub const astcenc_profile_ASTCENC_PRF_LDR: astcenc_profile = 1;
#[doc = " @brief The HDR RGB with LDR alpha color profile."]
pub const astcenc_profile_ASTCENC_PRF_HDR_RGB_LDR_A: astcenc_profile = 2;
#[doc = " @brief The HDR RGBA color profile."]
pub const astcenc_profile_ASTCENC_PRF_HDR: astcenc_profile = 3;
#[doc = " @brief A codec color profile."]
pub type astcenc_profile = ::std::os::raw::c_int;
#[doc = " @brief The fastest, lowest quality, search preset."]
pub const ASTCENC_PRE_FASTEST: f32 = 0.0;
#[doc = " @brief The fast search preset."]
pub const ASTCENC_PRE_FAST: f32 = 10.0;
#[doc = " @brief The medium quality search preset."]
pub const ASTCENC_PRE_MEDIUM: f32 = 60.0;
#[doc = " @brief The thorough quality search preset."]
pub const ASTCENC_PRE_THOROUGH: f32 = 98.0;
#[doc = " @brief The thorough quality search preset."]
pub const ASTCENC_PRE_VERYTHOROUGH: f32 = 99.0;
#[doc = " @brief The exhaustive, highest quality, search preset."]
pub const ASTCENC_PRE_EXHAUSTIVE: f32 = 100.0;
#[doc = " @brief Select the red component."]
pub const astcenc_swz_ASTCENC_SWZ_R: astcenc_swz = 0;
#[doc = " @brief Select the green component."]
pub const astcenc_swz_ASTCENC_SWZ_G: astcenc_swz = 1;
#[doc = " @brief Select the blue component."]
pub const astcenc_swz_ASTCENC_SWZ_B: astcenc_swz = 2;
#[doc = " @brief Select the alpha component."]
pub const astcenc_swz_ASTCENC_SWZ_A: astcenc_swz = 3;
#[doc = " @brief Use a constant zero component."]
pub const astcenc_swz_ASTCENC_SWZ_0: astcenc_swz = 4;
#[doc = " @brief Use a constant one component."]
pub const astcenc_swz_ASTCENC_SWZ_1: astcenc_swz = 5;
#[doc = " @brief Use a reconstructed normal vector Z component."]
pub const astcenc_swz_ASTCENC_SWZ_Z: astcenc_swz = 6;
#[doc = " @brief A codec component swizzle selector."]
pub type astcenc_swz = ::std::os::raw::c_int;
#[doc = " @brief A texel component swizzle."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct astcenc_swizzle {
    #[doc = " @brief The red component selector."]
    pub r: astcenc_swz,
    #[doc = " @brief The green component selector."]
    pub g: astcenc_swz,
    #[doc = " @brief The blue component selector."]
    pub b: astcenc_swz,
    #[doc = " @brief The alpha component selector."]
    pub a: astcenc_swz,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of astcenc_swizzle"][::std::mem::size_of::<astcenc_swizzle>() - 16usize];
    ["Alignment of astcenc_swizzle"][::std::mem::align_of::<astcenc_swizzle>() - 4usize];
    ["Offset of field: astcenc_swizzle::r"][::std::mem::offset_of!(astcenc_swizzle, r) - 0usize];
    ["Offset of field: astcenc_swizzle::g"][::std::mem::offset_of!(astcenc_swizzle, g) - 4usize];
    ["Offset of field: astcenc_swizzle::b"][::std::mem::offset_of!(astcenc_swizzle, b) - 8usize];
    ["Offset of field: astcenc_swizzle::a"][::std::mem::offset_of!(astcenc_swizzle, a) - 12usize];
};
#[doc = " @brief Unorm 8-bit data per component."]
pub const astcenc_type_ASTCENC_TYPE_U8: astcenc_type = 0;
#[doc = " @brief 16-bit float per component."]
pub const astcenc_type_ASTCENC_TYPE_F16: astcenc_type = 1;
#[doc = " @brief 32-bit float per component."]
pub const astcenc_type_ASTCENC_TYPE_F32: astcenc_type = 2;
#[doc = " @brief A texel component data format."]
pub type astcenc_type = ::std::os::raw::c_int;
#[doc = " @brief Function pointer type for compression progress reporting callback."]
pub type astcenc_progress_callback = ::std::option::Option<unsafe extern "C" fn(arg1: f32)>;
#[doc = " @brief Enable normal map compression.\n\n Input data will be treated a two component normal map, storing X and Y, and the codec will\n optimize for angular error rather than simple linear PSNR. In this mode the input swizzle should\n be e.g. rrrg (the default ordering for ASTC normals on the command line) or gggr (the ordering\n used by BC5n)."]
pub const ASTCENC_FLG_MAP_NORMAL: ::std::os::raw::c_uint = 1;
#[doc = " @brief Enable compression heuristics that assume use of decode_unorm8 decode mode.\n\n The decode_unorm8 decode mode rounds differently to the decode_fp16 decode mode, so enabling this\n flag during compression will allow the compressor to use the correct rounding when selecting\n encodings. This will improve the compressed image quality if your application is using the\n decode_unorm8 decode mode, but will reduce image quality if using decode_fp16.\n\n Note that LDR_SRGB images will always use decode_unorm8 for the RGB channels, irrespective of\n this setting."]
pub const ASTCENC_FLG_USE_DECODE_UNORM8: ::std::os::raw::c_uint = 2;
#[doc = " @brief Enable alpha weighting.\n\n The input alpha value is used for transparency, so errors in the RGB components are weighted by\n the transparency level. This allows the codec to more accurately encode the alpha value in areas\n where the color value is less significant."]
pub const ASTCENC_FLG_USE_ALPHA_WEIGHT: ::std::os::raw::c_uint = 4;
#[doc = " @brief Enable perceptual error metrics.\n\n This mode enables perceptual compression mode, which will optimize for perceptual error rather\n than best PSNR. Only some input modes support perceptual error metrics."]
pub const ASTCENC_FLG_USE_PERCEPTUAL: ::std::os::raw::c_uint = 8;
#[doc = " @brief Create a decompression-only context.\n\n This mode disables support for compression. This enables context allocation to skip some\n transient buffer allocation, resulting in lower memory usage."]
pub const ASTCENC_FLG_DECOMPRESS_ONLY: ::std::os::raw::c_uint = 16;
#[doc = " @brief Create a self-decompression context.\n\n This mode configures the compressor so that it is only guaranteed to be able to decompress images\n that were actually created using the current context. This is the common case for compression use\n cases, and setting this flag enables additional optimizations, but does mean that the context\n cannot reliably decompress arbitrary ASTC images."]
pub const ASTCENC_FLG_SELF_DECOMPRESS_ONLY: ::std::os::raw::c_uint = 32;
#[doc = " @brief Enable RGBM map compression.\n\n Input data will be treated as HDR data that has been stored in an LDR RGBM-encoded wrapper\n format. Data must be preprocessed by the user to be in LDR RGBM format before calling the\n compression function, this flag is only used to control the use of RGBM-specific heuristics and\n error metrics.\n\n IMPORTANT: The ASTC format is prone to bad failure modes with unconstrained RGBM data; very small\n M values can round to zero due to quantization and result in black or white pixels. It is highly\n recommended that the minimum value of M used in the encoding is kept above a lower threshold (try\n 16 or 32). Applying this threshold reduces the number of very dark colors that can be\n represented, but is still higher precision than 8-bit LDR.\n\n When this flag is set the value of @c rgbm_m_scale in the context must be set to the RGBM scale\n factor used during reconstruction. This defaults to 5 when in RGBM mode.\n\n It is recommended that the value of @c cw_a_weight is set to twice the value of the multiplier\n scale, ensuring that the M value is accurately encoded. This defaults to 10 when in RGBM mode,\n matching the default scale factor."]
pub const ASTCENC_FLG_MAP_RGBM: ::std::os::raw::c_uint = 64;
#[doc = " @brief The bit mask of all valid flags."]
pub const ASTCENC_ALL_FLAGS: ::std::os::raw::c_uint = 127;
#[doc = " @brief The config structure.\n\n This structure will initially be populated by a call to astcenc_config_init, but power users may\n modify it before calling astcenc_context_alloc. See astcenccli_toplevel_help.cpp for full user\n documentation of the power-user settings.\n\n Note for any settings which are associated with a specific color component, the value in the\n config applies to the component that exists after any compression data swizzle is applied."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct astcenc_config {
    #[doc = " @brief The color profile."]
    pub profile: astcenc_profile,
    #[doc = " @brief The set of set flags."]
    pub flags: ::std::os::raw::c_uint,
    #[doc = " @brief The ASTC block size X dimension."]
    pub block_x: ::std::os::raw::c_uint,
    #[doc = " @brief The ASTC block size Y dimension."]
    pub block_y: ::std::os::raw::c_uint,
    #[doc = " @brief The ASTC block size Z dimension."]
    pub block_z: ::std::os::raw::c_uint,
    #[doc = " @brief The red component weight scale for error weighting (-cw)."]
    pub cw_r_weight: f32,
    #[doc = " @brief The green component weight scale for error weighting (-cw)."]
    pub cw_g_weight: f32,
    #[doc = " @brief The blue component weight scale for error weighting (-cw)."]
    pub cw_b_weight: f32,
    #[doc = " @brief The alpha component weight scale for error weighting (-cw)."]
    pub cw_a_weight: f32,
    #[doc = " @brief The radius for any alpha-weight scaling (-a).\n\n It is recommended that this is set to 1 when using FLG_USE_ALPHA_WEIGHT on a texture that\n will be sampled using linear texture filtering to minimize color bleed out of transparent\n texels that are adjacent to non-transparent texels."]
    pub a_scale_radius: ::std::os::raw::c_uint,
    #[doc = " @brief The RGBM scale factor for the shared multiplier (-rgbm)."]
    pub rgbm_m_scale: f32,
    #[doc = " @brief The maximum number of partitions searched (-partitioncountlimit).\n\n Valid values are between 1 and 4."]
    pub tune_partition_count_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The maximum number of partitions searched (-2partitionindexlimit).\n\n Valid values are between 1 and 1024."]
    pub tune_2partition_index_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The maximum number of partitions searched (-3partitionindexlimit).\n\n Valid values are between 1 and 1024."]
    pub tune_3partition_index_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The maximum number of partitions searched (-4partitionindexlimit).\n\n Valid values are between 1 and 1024."]
    pub tune_4partition_index_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The maximum centile for block modes searched (-blockmodelimit).\n\n Valid values are between 1 and 100."]
    pub tune_block_mode_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The maximum iterative refinements applied (-refinementlimit).\n\n Valid values are between 1 and N; there is no technical upper limit\n but little benefit is expected after N=4."]
    pub tune_refinement_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The number of trial candidates per mode search (-candidatelimit).\n\n Valid values are between 1 and TUNE_MAX_TRIAL_CANDIDATES."]
    pub tune_candidate_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The number of trial partitionings per search (-2partitioncandidatelimit).\n\n Valid values are between 1 and TUNE_MAX_PARTITIONING_CANDIDATES."]
    pub tune_2partitioning_candidate_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The number of trial partitionings per search (-3partitioncandidatelimit).\n\n Valid values are between 1 and TUNE_MAX_PARTITIONING_CANDIDATES."]
    pub tune_3partitioning_candidate_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The number of trial partitionings per search (-4partitioncandidatelimit).\n\n Valid values are between 1 and TUNE_MAX_PARTITIONING_CANDIDATES."]
    pub tune_4partitioning_candidate_limit: ::std::os::raw::c_uint,
    #[doc = " @brief The dB threshold for stopping block search (-dblimit).\n\n This option is ineffective for HDR textures."]
    pub tune_db_limit: f32,
    #[doc = " @brief The amount of MSE overshoot needed to early-out trials.\n\n The first early-out is for 1 partition, 1 plane trials, where we try a minimal encode using\n the high probability block modes. This can short-cut compression for simple blocks.\n\n The second early-out is for refinement trials, where we can exit refinement once quality is\n reached."]
    pub tune_mse_overshoot: f32,
    #[doc = " @brief The threshold for skipping 3.1/4.1 trials (-2partitionlimitfactor).\n\n This option is further scaled for normal maps, so it skips less often."]
    pub tune_2partition_early_out_limit_factor: f32,
    #[doc = " @brief The threshold for skipping 4.1 trials (-3partitionlimitfactor).\n\n This option is further scaled for normal maps, so it skips less often."]
    pub tune_3partition_early_out_limit_factor: f32,
    #[doc = " @brief The threshold for skipping two weight planes (-2planelimitcorrelation).\n\n This option is ineffective for normal maps."]
    pub tune_2plane_early_out_limit_correlation: f32,
    #[doc = " @brief The config enable for the mode0 fast-path search.\n\n If this is set to TUNE_MIN_TEXELS_MODE0 or higher then the early-out fast mode0\n search is enabled. This option is ineffective for 3D block sizes."]
    pub tune_search_mode0_enable: f32,
    #[doc = " @brief The progress callback, can be @c nullptr.\n\n If this is specified the codec will peridocially report progress for\n compression as a percentage between 0 and 100. The callback is called from one\n of the compressor threads, so doing significant work in the callback will\n reduce compression performance."]
    pub progress_callback: astcenc_progress_callback,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of astcenc_config"][::std::mem::size_of::<astcenc_config>() - 120usize];
    ["Alignment of astcenc_config"][::std::mem::align_of::<astcenc_config>() - 8usize];
    ["Offset of field: astcenc_config::profile"]
        [::std::mem::offset_of!(astcenc_config, profile) - 0usize];
    ["Offset of field: astcenc_config::flags"]
        [::std::mem::offset_of!(astcenc_config, flags) - 4usize];
    ["Offset of field: astcenc_config::block_x"]
        [::std::mem::offset_of!(astcenc_config, block_x) - 8usize];
    ["Offset of field: astcenc_config::block_y"]
        [::std::mem::offset_of!(astcenc_config, block_y) - 12usize];
    ["Offset of field: astcenc_config::block_z"]
        [::std::mem::offset_of!(astcenc_config, block_z) - 16usize];
    ["Offset of field: astcenc_config::cw_r_weight"]
        [::std::mem::offset_of!(astcenc_config, cw_r_weight) - 20usize];
    ["Offset of field: astcenc_config::cw_g_weight"]
        [::std::mem::offset_of!(astcenc_config, cw_g_weight) - 24usize];
    ["Offset of field: astcenc_config::cw_b_weight"]
        [::std::mem::offset_of!(astcenc_config, cw_b_weight) - 28usize];
    ["Offset of field: astcenc_config::cw_a_weight"]
        [::std::mem::offset_of!(astcenc_config, cw_a_weight) - 32usize];
    ["Offset of field: astcenc_config::a_scale_radius"]
        [::std::mem::offset_of!(astcenc_config, a_scale_radius) - 36usize];
    ["Offset of field: astcenc_config::rgbm_m_scale"]
        [::std::mem::offset_of!(astcenc_config, rgbm_m_scale) - 40usize];
    ["Offset of field: astcenc_config::tune_partition_count_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_partition_count_limit) - 44usize];
    ["Offset of field: astcenc_config::tune_2partition_index_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_2partition_index_limit) - 48usize];
    ["Offset of field: astcenc_config::tune_3partition_index_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_3partition_index_limit) - 52usize];
    ["Offset of field: astcenc_config::tune_4partition_index_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_4partition_index_limit) - 56usize];
    ["Offset of field: astcenc_config::tune_block_mode_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_block_mode_limit) - 60usize];
    ["Offset of field: astcenc_config::tune_refinement_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_refinement_limit) - 64usize];
    ["Offset of field: astcenc_config::tune_candidate_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_candidate_limit) - 68usize];
    ["Offset of field: astcenc_config::tune_2partitioning_candidate_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_2partitioning_candidate_limit) - 72usize];
    ["Offset of field: astcenc_config::tune_3partitioning_candidate_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_3partitioning_candidate_limit) - 76usize];
    ["Offset of field: astcenc_config::tune_4partitioning_candidate_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_4partitioning_candidate_limit) - 80usize];
    ["Offset of field: astcenc_config::tune_db_limit"]
        [::std::mem::offset_of!(astcenc_config, tune_db_limit) - 84usize];
    ["Offset of field: astcenc_config::tune_mse_overshoot"]
        [::std::mem::offset_of!(astcenc_config, tune_mse_overshoot) - 88usize];
    ["Offset of field: astcenc_config::tune_2partition_early_out_limit_factor"]
        [::std::mem::offset_of!(astcenc_config, tune_2partition_early_out_limit_factor) - 92usize];
    ["Offset of field: astcenc_config::tune_3partition_early_out_limit_factor"]
        [::std::mem::offset_of!(astcenc_config, tune_3partition_early_out_limit_factor) - 96usize];
    ["Offset of field: astcenc_config::tune_2plane_early_out_limit_correlation"][::std::mem::offset_of!(
        astcenc_config,
        tune_2plane_early_out_limit_correlation
    ) - 100usize];
    ["Offset of field: astcenc_config::tune_search_mode0_enable"]
        [::std::mem::offset_of!(astcenc_config, tune_search_mode0_enable) - 104usize];
    ["Offset of field: astcenc_config::progress_callback"]
        [::std::mem::offset_of!(astcenc_config, progress_callback) - 112usize];
};
#[doc = " @brief An uncompressed 2D or 3D image.\n\n 3D image are passed in as an array of 2D slices. Each slice has identical\n size and color format."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct astcenc_image {
    #[doc = " @brief The X dimension of the image, in texels."]
    pub dim_x: ::std::os::raw::c_uint,
    #[doc = " @brief The Y dimension of the image, in texels."]
    pub dim_y: ::std::os::raw::c_uint,
    #[doc = " @brief The Z dimension of the image, in texels."]
    pub dim_z: ::std::os::raw::c_uint,
    #[doc = " @brief The data type per component."]
    pub data_type: astcenc_type,
    #[doc = " @brief The array of 2D slices, of length @c dim_z."]
    pub data: *mut *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of astcenc_image"][::std::mem::size_of::<astcenc_image>() - 24usize];
    ["Alignment of astcenc_image"][::std::mem::align_of::<astcenc_image>() - 8usize];
    ["Offset of field: astcenc_image::dim_x"]
        [::std::mem::offset_of!(astcenc_image, dim_x) - 0usize];
    ["Offset of field: astcenc_image::dim_y"]
        [::std::mem::offset_of!(astcenc_image, dim_y) - 4usize];
    ["Offset of field: astcenc_image::dim_z"]
        [::std::mem::offset_of!(astcenc_image, dim_z) - 8usize];
    ["Offset of field: astcenc_image::data_type"]
        [::std::mem::offset_of!(astcenc_image, data_type) - 12usize];
    ["Offset of field: astcenc_image::data"][::std::mem::offset_of!(astcenc_image, data) - 16usize];
};
#[doc = " @brief A block encoding metadata query result.\n\n If the block is an error block or a constant color block or an error block all fields other than\n the profile, block dimensions, and error/constant indicator will be zero."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct astcenc_block_info {
    #[doc = " @brief The block encoding color profile."]
    pub profile: astcenc_profile,
    #[doc = " @brief The number of texels in the X dimension."]
    pub block_x: ::std::os::raw::c_uint,
    #[doc = " @brief The number of texels in the Y dimension."]
    pub block_y: ::std::os::raw::c_uint,
    #[doc = " @brief The number of texel in the Z dimension."]
    pub block_z: ::std::os::raw::c_uint,
    #[doc = " @brief The number of texels in the block."]
    pub texel_count: ::std::os::raw::c_uint,
    #[doc = " @brief True if this block is an error block."]
    pub is_error_block: bool,
    #[doc = " @brief True if this block is a constant color block."]
    pub is_constant_block: bool,
    #[doc = " @brief True if this block is an HDR block."]
    pub is_hdr_block: bool,
    #[doc = " @brief True if this block uses two weight planes."]
    pub is_dual_plane_block: bool,
    #[doc = " @brief The number of partitions if not constant color."]
    pub partition_count: ::std::os::raw::c_uint,
    #[doc = " @brief The partition index if 2 - 4 partitions used."]
    pub partition_index: ::std::os::raw::c_uint,
    #[doc = " @brief The component index of the second plane if dual plane."]
    pub dual_plane_component: ::std::os::raw::c_uint,
    #[doc = " @brief The color endpoint encoding mode for each partition."]
    pub color_endpoint_modes: [::std::os::raw::c_uint; 4usize],
    #[doc = " @brief The number of color endpoint quantization levels."]
    pub color_level_count: ::std::os::raw::c_uint,
    #[doc = " @brief The number of weight quantization levels."]
    pub weight_level_count: ::std::os::raw::c_uint,
    #[doc = " @brief The number of weights in the X dimension."]
    pub weight_x: ::std::os::raw::c_uint,
    #[doc = " @brief The number of weights in the Y dimension."]
    pub weight_y: ::std::os::raw::c_uint,
    #[doc = " @brief The number of weights in the Z dimension."]
    pub weight_z: ::std::os::raw::c_uint,
    #[doc = " @brief The unpacked color endpoints for each partition."]
    pub color_endpoints: [[[f32; 4usize]; 2usize]; 4usize],
    #[doc = " @brief The per-texel interpolation weights for the block."]
    pub weight_values_plane1: [f32; 216usize],
    #[doc = " @brief The per-texel interpolation weights for the block."]
    pub weight_values_plane2: [f32; 216usize],
    #[doc = " @brief The per-texel partition assignments for the block."]
    pub partition_assignment: [u8; 216usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of astcenc_block_info"][::std::mem::size_of::<astcenc_block_info>() - 2144usize];
    ["Alignment of astcenc_block_info"][::std::mem::align_of::<astcenc_block_info>() - 4usize];
    ["Offset of field: astcenc_block_info::profile"]
        [::std::mem::offset_of!(astcenc_block_info, profile) - 0usize];
    ["Offset of field: astcenc_block_info::block_x"]
        [::std::mem::offset_of!(astcenc_block_info, block_x) - 4usize];
    ["Offset of field: astcenc_block_info::block_y"]
        [::std::mem::offset_of!(astcenc_block_info, block_y) - 8usize];
    ["Offset of field: astcenc_block_info::block_z"]
        [::std::mem::offset_of!(astcenc_block_info, block_z) - 12usize];
    ["Offset of field: astcenc_block_info::texel_count"]
        [::std::mem::offset_of!(astcenc_block_info, texel_count) - 16usize];
    ["Offset of field: astcenc_block_info::is_error_block"]
        [::std::mem::offset_of!(astcenc_block_info, is_error_block) - 20usize];
    ["Offset of field: astcenc_block_info::is_constant_block"]
        [::std::mem::offset_of!(astcenc_block_info, is_constant_block) - 21usize];
    ["Offset of field: astcenc_block_info::is_hdr_block"]
        [::std::mem::offset_of!(astcenc_block_info, is_hdr_block) - 22usize];
    ["Offset of field: astcenc_block_info::is_dual_plane_block"]
        [::std::mem::offset_of!(astcenc_block_info, is_dual_plane_block) - 23usize];
    ["Offset of field: astcenc_block_info::partition_count"]
        [::std::mem::offset_of!(astcenc_block_info, partition_count) - 24usize];
    ["Offset of field: astcenc_block_info::partition_index"]
        [::std::mem::offset_of!(astcenc_block_info, partition_index) - 28usize];
    ["Offset of field: astcenc_block_info::dual_plane_component"]
        [::std::mem::offset_of!(astcenc_block_info, dual_plane_component) - 32usize];
    ["Offset of field: astcenc_block_info::color_endpoint_modes"]
        [::std::mem::offset_of!(astcenc_block_info, color_endpoint_modes) - 36usize];
    ["Offset of field: astcenc_block_info::color_level_count"]
        [::std::mem::offset_of!(astcenc_block_info, color_level_count) - 52usize];
    ["Offset of field: astcenc_block_info::weight_level_count"]
        [::std::mem::offset_of!(astcenc_block_info, weight_level_count) - 56usize];
    ["Offset of field: astcenc_block_info::weight_x"]
        [::std::mem::offset_of!(astcenc_block_info, weight_x) - 60usize];
    ["Offset of field: astcenc_block_info::weight_y"]
        [::std::mem::offset_of!(astcenc_block_info, weight_y) - 64usize];
    ["Offset of field: astcenc_block_info::weight_z"]
        [::std::mem::offset_of!(astcenc_block_info, weight_z) - 68usize];
    ["Offset of field: astcenc_block_info::color_endpoints"]
        [::std::mem::offset_of!(astcenc_block_info, color_endpoints) - 72usize];
    ["Offset of field: astcenc_block_info::weight_values_plane1"]
        [::std::mem::offset_of!(astcenc_block_info, weight_values_plane1) - 200usize];
    ["Offset of field: astcenc_block_info::weight_values_plane2"]
        [::std::mem::offset_of!(astcenc_block_info, weight_values_plane2) - 1064usize];
    ["Offset of field: astcenc_block_info::partition_assignment"]
        [::std::mem::offset_of!(astcenc_block_info, partition_assignment) - 1928usize];
};
extern "C" {
    #[doc = " Populate a codec config based on default settings.\n\n Power users can edit the returned config struct to fine tune before allocating the context.\n\n @param      profile   Color profile.\n @param      block_x   ASTC block size X dimension.\n @param      block_y   ASTC block size Y dimension.\n @param      block_z   ASTC block size Z dimension.\n @param      quality   Search quality preset / effort level. Either an\n                       @c ASTCENC_PRE_* value, or a effort level between 0\n                       and 100. Performance is not linear between 0 and 100.\n\n @param      flags     A valid set of @c ASTCENC_FLG_* flag bits.\n @param[out] config    Output config struct to populate.\n\n @return @c ASTCENC_SUCCESS on success, or an error if the inputs are invalid\n either individually, or in combination."]
    #[link_name = "\u{1}?astcenc_config_init@@YA?AW4astcenc_error@@W4astcenc_profile@@IIIMIPEAUastcenc_config@@@Z"]
    pub fn astcenc_config_init(
        profile: astcenc_profile,
        block_x: ::std::os::raw::c_uint,
        block_y: ::std::os::raw::c_uint,
        block_z: ::std::os::raw::c_uint,
        quality: f32,
        flags: ::std::os::raw::c_uint,
        config: *mut astcenc_config,
    ) -> astcenc_error;
}
extern "C" {
    #[doc = " @brief Allocate a new codec context based on a config.\n\n This function allocates all of the memory resources and threads needed by the codec. This can be\n slow, so it is recommended that contexts are reused to serially compress or decompress multiple\n images to amortize setup cost.\n\n Contexts can be allocated to support only decompression using the @c ASTCENC_FLG_DECOMPRESS_ONLY\n flag when creating the configuration. The compression functions will fail if invoked. For a\n decompress-only library build the @c ASTCENC_FLG_DECOMPRESS_ONLY flag must be set when creating\n any context.\n\n @param[in]  config         Codec config.\n @param      thread_count   Thread count to configure for.\n @param[out] context        Location to store an opaque context pointer.\n\n @return @c ASTCENC_SUCCESS on success, or an error if context creation failed."]
    #[link_name = "\u{1}?astcenc_context_alloc@@YA?AW4astcenc_error@@PEBUastcenc_config@@IPEAPEAUastcenc_context@@@Z"]
    pub fn astcenc_context_alloc(
        config: *const astcenc_config,
        thread_count: ::std::os::raw::c_uint,
        context: *mut *mut astcenc_context,
    ) -> astcenc_error;
}
extern "C" {
    #[doc = " @brief Compress an image.\n\n A single context can only compress or decompress a single image at a time.\n\n For a context configured for multi-threading, any set of the N threads can call this function.\n Work will be dynamically scheduled across the threads available. Each thread must have a unique\n @c thread_index.\n\n @param         context        Codec context.\n @param[in,out] image          An input image, in 2D slices.\n @param         swizzle        Compression data swizzle, applied before compression.\n @param[out]    data_out       Pointer to output data array.\n @param         data_len       Length of the output data array.\n @param         thread_index   Thread index [0..N-1] of calling thread.\n\n @return @c ASTCENC_SUCCESS on success, or an error if compression failed."]
    #[link_name = "\u{1}?astcenc_compress_image@@YA?AW4astcenc_error@@PEAUastcenc_context@@PEAUastcenc_image@@PEBUastcenc_swizzle@@PEAE_KI@Z"]
    pub fn astcenc_compress_image(
        context: *mut astcenc_context,
        image: *mut astcenc_image,
        swizzle: *const astcenc_swizzle,
        data_out: *mut u8,
        data_len: usize,
        thread_index: ::std::os::raw::c_uint,
    ) -> astcenc_error;
}
extern "C" {
    #[doc = " @brief Reset the codec state for a new compression.\n\n The caller is responsible for synchronizing threads in the worker thread pool. This function must\n only be called when all threads have exited the @c astcenc_compress_image() function for image N,\n but before any thread enters it for image N + 1.\n\n Calling this is not required (but won't hurt), if the context is created for single threaded use.\n\n @param context   Codec context.\n\n @return @c ASTCENC_SUCCESS on success, or an error if reset failed."]
    #[link_name = "\u{1}?astcenc_compress_reset@@YA?AW4astcenc_error@@PEAUastcenc_context@@@Z"]
    pub fn astcenc_compress_reset(context: *mut astcenc_context) -> astcenc_error;
}
extern "C" {
    #[doc = " @brief Decompress an image.\n\n @param         context        Codec context.\n @param[in]     data           Pointer to compressed data.\n @param         data_len       Length of the compressed data, in bytes.\n @param[in,out] image_out      Output image.\n @param         swizzle        Decompression data swizzle, applied after decompression.\n @param         thread_index   Thread index [0..N-1] of calling thread.\n\n @return @c ASTCENC_SUCCESS on success, or an error if decompression failed."]
    #[link_name = "\u{1}?astcenc_decompress_image@@YA?AW4astcenc_error@@PEAUastcenc_context@@PEBE_KPEAUastcenc_image@@PEBUastcenc_swizzle@@I@Z"]
    pub fn astcenc_decompress_image(
        context: *mut astcenc_context,
        data: *const u8,
        data_len: usize,
        image_out: *mut astcenc_image,
        swizzle: *const astcenc_swizzle,
        thread_index: ::std::os::raw::c_uint,
    ) -> astcenc_error;
}
extern "C" {
    #[doc = " @brief Reset the codec state for a new decompression.\n\n The caller is responsible for synchronizing threads in the worker thread pool. This function must\n only be called when all threads have exited the @c astcenc_decompress_image() function for image\n N, but before any thread enters it for image N + 1.\n\n Calling this is not required (but won't hurt), if the context is created for single threaded use.\n\n @param context   Codec context.\n\n @return @c ASTCENC_SUCCESS on success, or an error if reset failed."]
    #[link_name = "\u{1}?astcenc_decompress_reset@@YA?AW4astcenc_error@@PEAUastcenc_context@@@Z"]
    pub fn astcenc_decompress_reset(context: *mut astcenc_context) -> astcenc_error;
}
extern "C" {
    #[doc = " Free the compressor context.\n\n @param context   The codec context."]
    #[link_name = "\u{1}?astcenc_context_free@@YAXPEAUastcenc_context@@@Z"]
    pub fn astcenc_context_free(context: *mut astcenc_context);
}
extern "C" {
    #[doc = " @brief Provide a high level summary of a block's encoding.\n\n This feature is primarily useful for codec developers but may be useful for developers building\n advanced content packaging pipelines.\n\n @param context   Codec context.\n @param data      One block of compressed ASTC data.\n @param info      The output info structure to populate.\n\n @return @c ASTCENC_SUCCESS if the block was decoded, or an error otherwise. Note that this\n         function will return success even if the block itself was an error block encoding, as the\n         decode was correctly handled."]
    #[link_name = "\u{1}?astcenc_get_block_info@@YA?AW4astcenc_error@@PEAUastcenc_context@@QEBEPEAUastcenc_block_info@@@Z"]
    pub fn astcenc_get_block_info(
        context: *mut astcenc_context,
        data: *const u8,
        info: *mut astcenc_block_info,
    ) -> astcenc_error;
}
extern "C" {
    #[doc = " @brief Get a printable string for specific status code.\n\n @param status   The status value.\n\n @return A human readable nul-terminated string."]
    #[link_name = "\u{1}?astcenc_get_error_string@@YAPEBDW4astcenc_error@@@Z"]
    pub fn astcenc_get_error_string(status: astcenc_error) -> *const ::std::os::raw::c_char;
}
