Name:           dedbg
Version:        0.1.0
Release:        1%{?dist}
Summary:        Find and remove dbg! macros in rust code

License:        MIT
# URL:            
Source0:        dedbg-0.1.0.tgz

# BuildRequires:  
# Requires:       

%define _build_id_links none

%description
removes dbg! rust macros from source code

%prep
%autosetup


%build
cargo build --release

%install
# %make_install

mkdir -p %{buildroot}/%{_bindir}
install -m 755 target/release/dedbg %{buildroot}/%{_bindir}/dedbg

%files
%{_bindir}/dedbg

%changelog
* Thu Sep 15 2022 eulegang <eulegang@eulegang.dev>
- 
