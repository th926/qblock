<?php
/**
 * CAP_PLACEHOLDER Block
 * @param array         $block      The block settings and attributes.
 * @param string        $content    The block inner HTML (empty)
 * @param bool          $is_preview True during AJAX preview.
 * @param (int|string)  $post_id    The post ID this block is saved to.
 */

$extra_classes = "";
?>
<section <?php create_block_id($block); create_block_classes($block, $extra_classes); ?>>

</section>
