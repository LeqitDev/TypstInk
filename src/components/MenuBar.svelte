<script lang="ts">
    import * as Menubar from "$lib/components/ui/menubar";
</script>

<div class="w-full flex-shrink">
    <Menubar.Root class="border-t-0 border-l-0 border-r-0 rounded-none">
        <Menubar.Menu>
            <Menubar.Trigger class="hover:cursor-pointer">File</Menubar.Trigger>
            <Menubar.Content>
                <Menubar.Item>
                    New Tab
                    <Menubar.Shortcut>⌘T</Menubar.Shortcut>
                </Menubar.Item>
                <Menubar.Item>New Window</Menubar.Item>
                <Menubar.Separator />
                <Menubar.Item>Share</Menubar.Item>
                <Menubar.Separator />
                <Menubar.Item>Print</Menubar.Item>
            </Menubar.Content>
        </Menubar.Menu>
    </Menubar.Root>
</div>